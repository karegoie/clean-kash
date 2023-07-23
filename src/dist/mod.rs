use std::collections::HashMap;
use kodama::{Method, linkage, Dendrogram};
use serde_pickle;

// l1 norm of two vectors
fn l1_norm(v1: &Vec<usize>, v2: &Vec<usize>) -> f32 {
    let mut sum = 0.0;
    for i in 0..v1.len() {
        sum += (v1[i] as f32 - v2[i] as f32).abs();
    }
    sum
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

#[cfg(test)]
mod test {
    #[test]
    fn test_l1_norm() {
        use super::*;
        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5, 6];
        assert_eq!(l1_norm(&v1, &v2), 9.0);
    }

    #[test]
    fn test_build_condensed_distance_matrix() {
        use super::*;
        let mut target: HashMap<Vec<u8>, Vec<usize>> = HashMap::new();
        target.insert(vec![99, 100, 101], vec![1, 2, 3]);
        target.insert(vec![98,97, 96], vec![4, 5, 6]);
        let (names, condensed) = build_condensed_distance_matrix(&target);
        assert_eq!(names, vec![vec![99, 100, 101], vec![98, 97, 96]]);
        assert_eq!(condensed, vec![9.0]);
    }
    #[test]
    fn test_dendrogram() {
        use super::*;
        let mut target: HashMap<Vec<u8>, Vec<usize>> = HashMap::new();
        target.insert(vec![99, 100, 101], vec![1, 2, 3]);
        target.insert(vec![98,97, 96], vec![4, 5, 6]);
        let (names, condensed) = build_condensed_distance_matrix(&target);
        let dend = dendrogram(target.len(), condensed);
        to_pickle_with_serde_names(names);
        to_pickle_with_serde_dend(dend);
        println!("{:?}", dend);
    }
}