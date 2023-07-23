use std::collections::HashMap;
use kodama::{Method, linkage, Dendrogram};
use serde_pickle;

// l1 norm of two vectors
fn l1_norm(v1: &Vec<usize>, v2: &Vec<usize>) -> f32 {
    let mut sum = 0.0;
    for i in 0..v1.len() {
        sum += (v1[i] as f32 - v2[i] as f32).abs();
    }
    // logarithm of sum if sum is not 0
    if sum != 0.0 {
        sum.log2()
    } else {
        0.0
    }
}

pub fn build_condensed_distance_matrix(target:&HashMap<Vec<u8>, Vec<usize>>) -> (Vec<Vec<u8>>, Vec<f32>) {
    let mut condensed = vec![];
    let mut names = vec![];
    for key in target.keys() {
        names.push(key.to_vec());
    }
    for row in 0..target.len() {
        for col in row + 1..target.len() {
            condensed.push(l1_norm(&target[&names[row]], &target[&names[col]]).into());
        }
    }
    (names, condensed)
}

pub fn dendrogram(length: usize, mut condens: Vec<f32>) -> Dendrogram<f32>{
    let dend = linkage(&mut condens, length, Method::Average);
    dend
}

pub fn to_pickle_with_serde_names(names: Vec<Vec<u8>>) {
    // convert names to &str
    let names_str: Vec<&str> = names.iter().map(|x| std::str::from_utf8(x).unwrap()).collect::<Vec<&str>>();
    let serialized = serde_pickle::to_vec(&names_str, Default::default()).unwrap();
    std::fs::write("data/names.pkl", serialized).unwrap();
}

pub fn to_pickle_with_serde_dend(dend: Dendrogram<f32>) {
    let serialized = serde_pickle::to_vec(&format!("{:?}", dend), Default::default()).unwrap();
    std::fs::write("data/dendrogram.pkl", serialized).unwrap();
}