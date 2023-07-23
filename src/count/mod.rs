use needletail::*;
use std::collections::{HashMap, BTreeMap, HashSet};
use rayon::prelude::*;
use std::str;

pub fn read_and_count_parallel(params: &HashMap<&str, &str>) -> HashMap<Vec<u8>, Vec<usize>> {
    let filename = params["file"];
    let mut reader = parse_fastx_file(&filename).expect("valid path/file");
    let mut kmer_count: HashMap<Vec<u8>, HashMap<Vec<u8>, usize>> = HashMap::new();

    // Save k-mer for each sequence in the HashMap
    while let Some(record) = reader.next() {
        let id = record.clone().expect("invalid record").id().to_owned();
        let seqrec = record.clone().expect("invalid record");
        let norm_seq = seqrec.normalize(false);
        let kmer_count_per_seq: HashMap<Vec<u8>, usize> = norm_seq
            .canonical_kmers(params["kmer"].parse().unwrap(), &norm_seq)
            .map(|(_, kmer, _)| kmer.to_owned())
            .collect::<Vec<Vec<u8>>>()
            .par_iter() // Parallelize the iteration
            .fold(
                || HashMap::new(),
                |mut acc, kmer| {
                    acc.entry(kmer.clone()).and_modify(|e| *e += 1).or_insert(1);
                    acc
                },
            )
            .reduce(
                || HashMap::new(),
                |mut acc1, acc2| {
                    for (kmer, count) in acc2 {
                        acc1.entry(kmer).and_modify(|e| *e += count).or_insert(count);
                    }
                    acc1
                },
            );
        kmer_count.insert(id, kmer_count_per_seq);
    }

    let common_kmers = get_common_kmers(&kmer_count);
    //for kmer in &common_kmers {
    //    println!("{}", str::from_utf8(kmer).unwrap());
    //}

    let extracted = extract_common_kmers(kmer_count, &common_kmers);
    extracted
}

fn get_common_kmers(kmer_count: &HashMap<Vec<u8>, HashMap<Vec<u8>, usize>>) -> HashSet<Vec<u8>> {
    // Get the first k-mer from the first sequence
    let mut first_kmers: Vec<Vec<u8>> = Vec::new();
    for (_, kmer_count_per_seq) in kmer_count {
        for kmer in kmer_count_per_seq.keys() {
            first_kmers.push(kmer.to_owned());
        }
    }

    // Get the common k-mers
    let mut common_kmers: HashSet<Vec<u8>> = HashSet::new();
    for kmer in first_kmers {
        let mut is_common = true;
        for (_, kmer_count_per_seq) in kmer_count {
            if !kmer_count_per_seq.contains_key(&kmer) {
                is_common = false;
            }
        }
        if is_common {
            common_kmers.insert(kmer);
        }
    }
    common_kmers
}

fn extract_common_kmers(
    kmer_count: HashMap<Vec<u8>, HashMap<Vec<u8>, usize>>,
    common_kmers: &HashSet<Vec<u8>>,
) -> HashMap<Vec<u8>, Vec<usize>> {

    // sort kmer_count_per_seq with BTreeMap
    let mut sorted_kmer_count: HashMap<Vec<u8>, BTreeMap<Vec<u8>, usize>> = HashMap::new();
    for (id, kmer_count_per_seq) in kmer_count {
        let mut sorted: BTreeMap<Vec<u8>, usize> = BTreeMap::new();
        for kmer in common_kmers {
            sorted.insert(kmer.to_owned(), kmer_count_per_seq[kmer]);
        }
        sorted_kmer_count.insert(id, sorted);
    }

    let mut extracted: HashMap<Vec<u8>, Vec<usize>> = HashMap::new();
    for (id, kmer_count_per_seq) in sorted_kmer_count {
        let mut counts: Vec<usize> = Vec::new();
        for kmer in common_kmers {
            counts.push(kmer_count_per_seq[kmer]);
        }
        extracted.insert(id, counts);
    }
    extracted
}