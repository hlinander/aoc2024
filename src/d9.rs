use std::{
    collections::VecDeque,
    io::{stdin, Read},
};

use itertools::Itertools;

pub fn day9() {
    day9_part1();
    day9_part2();
}

pub fn day9_part1() {
    let mut stdin = stdin();
    let data = crate::load_data("data/9").unwrap();
    let fs: Vec<u8> = data
        .chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect();
    let files = fs.iter().cloned().step_by(2).collect_vec();
    let mut free: VecDeque<u8> = fs.iter().cloned().skip(1).step_by(2).collect();
    // let mut free_it = free.into_iter();
    let mut files_it = files.clone().into_iter().enumerate();
    let mut files_rev_it = files.clone().into_iter().enumerate().rev();
    let mut compact: Vec<(usize, u8)> = Vec::new();

    let mut current_rev_file = None;
    let mut current_free_bytes = 0u8;
    loop {
        if current_free_bytes <= 0 {
            if let Some(next_file) = files_it.next() {
                if let Some((rev_file_idx, bytes)) = current_rev_file {
                    if rev_file_idx > next_file.0 {
                        compact.push(next_file);
                    }
                } else {
                    compact.push(next_file);
                }
            } else {
                println!("No more files...");
                break;
            }
            if let Some(next_free) = free.pop_front() {
                current_free_bytes = next_free;
            } else {
                if let Some((rev_file_idx, bytes)) = current_rev_file {
                    compact.push((rev_file_idx, bytes));
                };
                break;
            }
        }

        if current_rev_file.is_none() {
            if let Some(rev_file) = files_rev_it.next() {
                current_rev_file = Some(rev_file);
            } else {
                // No more reverse files left, we probably shoudn't be here?
                panic!();
            }
        }
        let Some((rev_file_idx, bytes)) = &mut current_rev_file else {
            panic!();
        };

        if *bytes <= current_free_bytes {
            compact.push((*rev_file_idx, *bytes));
            current_free_bytes -= *bytes;
            current_rev_file = None;
            free.pop_back();
        } else {
            compact.push((*rev_file_idx, current_free_bytes));
            *bytes -= current_free_bytes;
            current_free_bytes = 0;
        }
    }
    // println!("{:?}", compact);

    let mut checksum = 0;
    let mut cidx = 0;
    for (file_idx, bytes) in &compact {
        for _ in 0..*bytes {
            checksum += cidx * file_idx;
            cidx += 1;
        }
    }
    println!("{}", checksum);
}

#[derive(Debug, Clone)]
enum Type {
    File(usize, u8),
    Free(u8),
}

pub fn day9_part2() {
    let mut stdin = stdin();
    let data = crate::load_data("data/9").unwrap();

    let fs: Vec<u8> = data
        .chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect();
    let mut compact = fs
        .into_iter()
        .enumerate()
        .map(|(idx, bytes)| {
            if idx % 2 == 0 {
                Type::File(idx / 2, bytes)
            } else {
                Type::Free(bytes)
            }
        })
        .collect_vec();
    let max_idx = compact
        .iter()
        .filter_map(|typ| match typ {
            Type::File(idx, _) => Some(*idx),
            Type::Free(_) => None,
        })
        .max()
        .unwrap();

    let mut current_file_idx = max_idx;
    while current_file_idx > 0 {
        let (block_idx, Type::File(file_idx, size)) = compact
            .iter()
            // .rev()
            .find_position(|el| match el {
                Type::File(idx, _) => *idx == current_file_idx,
                Type::Free(_) => false,
            })
            .unwrap()
        else {
            panic!()
        };
        let size = *size;
        if let Some((free_block_idx, Type::Free(free_size))) =
            compact.iter().cloned().find_position(|el| match el {
                Type::File(_, _) => false,
                Type::Free(free_bytes) => *free_bytes >= size,
            })
        {
            if free_block_idx < block_idx {
                compact[free_block_idx] = Type::File(*file_idx, size);
                compact[block_idx] = Type::Free(size);
                if free_size > size {
                    compact.insert(free_block_idx + 1, Type::Free(free_size - size));
                }
            }
        }
        current_file_idx -= 1;
    }
    // render_blocks(&compact);

    let mut checksum = 0;
    let mut cidx = 0;
    for el in &compact {
        match el {
            Type::File(file_idx, bytes) => {
                for _ in 0..*bytes {
                    checksum += cidx * file_idx;
                    cidx += 1;
                }
            }
            Type::Free(size) => {
                cidx += *size as usize;
            }
        }
    }
    println!("Part 2 checksum {}", checksum);
}

fn render_blocks(compact: &Vec<Type>) {
    for el in compact {
        match el {
            Type::File(idx, size) => {
                let block_str = (0..*size).map(|_el| format!("{}", idx)).join("");
                print!("{}", block_str);
            }
            Type::Free(size) => {
                let block_str = (0..*size).map(|_el| format!(".")).join("");
                print!("{}", block_str);
            }
        }
    }
    print!("\n");
}
