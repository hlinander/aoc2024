use std::{
    collections::HashSet,
    io::{stdin, Read},
};

use itertools::Itertools;

fn to_matrix(data: String) -> Vec<Vec<char>> {
    data.split("\n")
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn get_start_pos(matrix: &Vec<Vec<char>>) -> Option<(i32, i32)> {
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == '^' {
                return Some((row as i32, col as i32));
            }
        }
    }
    None
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            print!("{}", matrix[row][col]);
        }
        print!("\n");
    }
}

pub fn day6() {
    day6_part1();
    day6_part2();
}

pub fn day6_part1() {
    let mut stdin = stdin();
    let data = crate::load_data("data/6").unwrap();
    let matrix = to_matrix(data);
    let mut pos = get_start_pos(&matrix).unwrap();
    let mut dir = (-1, 0);
    let matrix = matrix
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|el| if el == '^' { '.' } else { el })
                .collect_vec()
        })
        .collect_vec();
    let mut visited = HashSet::new();
    while pos.0 + dir.0 >= 0
        && pos.0 + dir.0 < matrix.len() as i32
        && pos.1 + dir.1 >= 0
        && pos.1 + dir.1 < matrix[0].len() as i32
    {
        visited.insert(pos.clone());
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if matrix[new_pos.0 as usize][new_pos.1 as usize] == '#' {
            // (-1, 0) -> (0, 1)
            dir = (dir.1, -dir.0);
            continue;
        }
        pos = new_pos;
        let mut vis_matrix = matrix.clone();
        vis_matrix[pos.0 as usize][pos.1 as usize] = 'X';
        // print_matrix(&vis_matrix);
        // let _ = stdin.read(&mut [0u8]).unwrap();
    }
    visited.insert(pos.clone());
    println!("Day 6 part 1: {}", visited.len());
}

pub fn day6_part2() {
    let data = crate::load_data("data/6").unwrap();
    let matrix = to_matrix(data);
    let pos = get_start_pos(&matrix).unwrap();
    let matrix = matrix
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|el| if el == '^' { '.' } else { el })
                .collect_vec()
        })
        .collect_vec();
    let mut num_loops = 0usize;
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == '.' && !(row as i32 == pos.0 && col as i32 == pos.1) {
                if is_loop(&matrix, pos.clone(), (row as i32, col as i32)) {
                    num_loops += 1;
                }
            }
        }
    }
    println!("Day 6 part 2: {}", num_loops);
}

fn is_loop(matrix: &Vec<Vec<char>>, pos: (i32, i32), new_obs: (i32, i32)) -> bool {
    let mut visited_and_dir = HashSet::new();
    let mut dir = (-1, 0);
    let mut pos = pos;
    let mut matrix = matrix.clone();
    matrix[new_obs.0 as usize][new_obs.1 as usize] = '#';
    while pos.0 + dir.0 >= 0
        && pos.0 + dir.0 < matrix.len() as i32
        && pos.1 + dir.1 >= 0
        && pos.1 + dir.1 < matrix[0].len() as i32
    {
        visited_and_dir.insert((pos.clone(), dir.clone()));
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if matrix[new_pos.0 as usize][new_pos.1 as usize] == '#' {
            dir = (dir.1, -dir.0);
            continue;
        }
        pos = new_pos;
        if visited_and_dir.contains(&(pos, dir)) {
            return true;
        }
    }
    false
}
