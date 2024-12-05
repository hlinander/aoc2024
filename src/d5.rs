use std::cmp::Ordering;

use itertools::Itertools;

pub fn day5() {
    let data = crate::load_data("data/5").unwrap();
    if let Some((ordering, lists)) = data.split_once("\n\n") {
        let ords = ordering
            .trim()
            .split("\n")
            .map(|line| line.split_once("|").unwrap())
            .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
            .collect_vec();
        let lists = lists
            .trim()
            .split("\n")
            .map(|list| {
                list.split(",")
                    .map(|element| element.parse::<i32>().unwrap())
                    .collect_vec()
            })
            .collect_vec();
        let mut sum = 0;
        for list in &lists {
            let mut sorted = list.clone();
            sorted.sort_by(|&a, &b| {
                if ords.contains(&(a, b)) {
                    Ordering::Less
                } else if ords.contains(&(b, a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            if sorted == *list {
                sum += list[list.len() / 2];
            }
        }
        println!("Part 1: {}", sum);

        let mut sum = 0;
        for list in &lists {
            let mut sorted = list.clone();
            sorted.sort_by(|&a, &b| {
                if ords.contains(&(a, b)) {
                    Ordering::Less
                } else if ords.contains(&(b, a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            if sorted != *list {
                sum += sorted[list.len() / 2];
            }
        }
        println!("Part 2: {}", sum);
    }
}
