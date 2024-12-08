use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap,
};

use itertools::Itertools;

pub fn day8() {
    let data = crate::load_data("data/8").unwrap();
    let matrix = to_matrix(data);
    let mut freqs: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            match matrix[row][col] {
                '.' => {}
                x => {
                    let entry = freqs.entry(x);
                    match entry {
                        Occupied(mut entry) => {
                            entry.get_mut().push((row as i32, col as i32));
                        }
                        Vacant(entry) => {
                            entry.insert(vec![(row as i32, col as i32)]);
                        }
                    }
                }
            }
        }
    }
    // Part 1
    let mut resonances = vec![vec![false; matrix[0].len()]; matrix.len()];
    for freq in freqs.values() {
        for pair in freq.iter().combinations(2) {
            let mut pair2 = pair.clone();
            mark_resonance(pair, &matrix, &mut resonances);
            pair2.reverse();
            mark_resonance(pair2, &matrix, &mut resonances);
        }
    }
    let n_resonances = resonances.into_iter().flatten().filter(|&x| x).count();
    println!("{} resonances", n_resonances);

    // Part 2
    let mut resonances = vec![vec![false; matrix[0].len()]; matrix.len()];
    for freq in freqs.values() {
        for pair in freq.iter().combinations(2) {
            let mut pair2 = pair.clone();
            mark_harmonic_resonance(pair, &matrix, &mut resonances);
            pair2.reverse();
            mark_harmonic_resonance(pair2, &matrix, &mut resonances);
        }
    }
    let n_resonances = resonances.into_iter().flatten().filter(|&x| x).count();
    println!("{} harmonic resonances", n_resonances);
}

fn mark_resonance(
    pair: Vec<&(i32, i32)>,
    matrix: &Vec<Vec<char>>,
    resonances: &mut Vec<Vec<bool>>,
) {
    let o1 = pair[0];
    let v = (pair[1].0 - o1.0, pair[1].1 - o1.1);
    let resonance = (o1.0 + 2 * v.0, o1.1 + 2 * v.1);
    if resonance.0 >= 0
        && resonance.0 < matrix.len() as i32
        && resonance.1 >= 0
        && resonance.1 < matrix[0].len() as i32
    {
        resonances[resonance.0 as usize][resonance.1 as usize] = true;
    }
}

fn mark_harmonic_resonance(
    pair: Vec<&(i32, i32)>,
    matrix: &Vec<Vec<char>>,
    resonances: &mut Vec<Vec<bool>>,
) {
    let o1 = pair[0];
    let v = (pair[1].0 - o1.0, pair[1].1 - o1.1);
    let mut resonance = o1.clone(); //(o1.0 + 2 * v.0, o1.1 + 2 * v.1);
    while resonance.0 >= 0
        && resonance.0 < matrix.len() as i32
        && resonance.1 >= 0
        && resonance.1 < matrix[0].len() as i32
    {
        resonances[resonance.0 as usize][resonance.1 as usize] = true;
        resonance.0 += v.0;
        resonance.1 += v.1;
    }
}
fn to_matrix(data: String) -> Vec<Vec<char>> {
    data.split("\n")
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}
