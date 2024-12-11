use std::collections::HashMap;

use itertools::Itertools;

pub fn day11() {
    let data = crate::load_data("data/11").unwrap();
    let stones: HashMap<u64, usize> = data
        .split(" ")
        .map(|stone| stone.parse::<u64>().unwrap())
        .map(|stone| (stone, 1))
        .collect();
    blink(stones.clone(), 25);
    blink(stones, 75);
}

fn blink(mut stones: HashMap<u64, usize>, n_blinks: usize) {
    for _ in 0..n_blinks {
        let mut new_stones: HashMap<u64, usize> = HashMap::new();
        for (&key, &val) in stones.iter() {
            let digits = format!("{}", key).chars().collect_vec();
            let n_digits = digits.len();
            if key == 0 {
                new_stones.entry(1).and_modify(|e| *e += val).or_insert(val);
            } else if n_digits % 2 == 0 {
                let (n1, n2) = digits.split_at(n_digits / 2);
                let n1 = n1.into_iter().collect::<String>().parse::<u64>().unwrap();
                let n2 = n2.into_iter().collect::<String>().parse::<u64>().unwrap();
                new_stones
                    .entry(n1)
                    .and_modify(|el| *el += val)
                    .or_insert(val);
                new_stones
                    .entry(n2)
                    .and_modify(|el| *el += val)
                    .or_insert(val);
            } else {
                new_stones
                    .entry(key * 2024)
                    .and_modify(|el| *el += val)
                    .or_insert(val);
            }
        }
        stones = new_stones;
    }
    let n_stones: usize = stones.values().sum();
    println!("{n_stones}");
}
