fn main() {
    todo!()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_total() {
        use clean_kash::count;
        use clean_kash::dist;
        use std::collections::HashMap;
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("file", "data/cotton.test.minimal.fa");
        params.insert("kmer", "17");
        let extracted = count::read_and_count_parallel(&params);
        let (names, condensed) = dist::build_condensed_distance_matrix(&extracted);
        let dend = dist::dendrogram(names.len(), condensed);
        dist::to_pickle_with_serde_names(names);
        dist::to_pickle_with_serde_dend(dend);
    }
}
