use itertools::Itertools;

pub fn day10() {
    let data = crate::load_data("data/10").unwrap();
    let matrix = to_matrix(data);
    let matrix: Vec<Vec<i8>> = matrix
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|c| format!("{c}").parse::<i8>().unwrap())
                .collect()
        })
        .collect();
    let mut paths: Vec<Vec<(i32, i32)>> = Vec::new();

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == 0 {
                paths.push(vec![(row as i32, col as i32)]);
            }
        }
    }
    // println!("{}", paths.len());

    let mut updated = true;
    while updated {
        updated = false;
        let mut new_paths: Vec<Vec<(i32, i32)>> = Vec::new();
        for path in &paths {
            let curr_pos = path[path.len() - 1];
            for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let next = (curr_pos.0 + dir.0, curr_pos.1 + dir.1);
                if next.0 >= 0
                    && next.0 < matrix.len() as i32
                    && next.1 >= 0
                    && next.1 < matrix[0].len() as i32
                {
                    if matrix[next.0 as usize][next.1 as usize]
                        - matrix[curr_pos.0 as usize][curr_pos.1 as usize]
                        == 1
                    {
                        if !path.contains(&next) {
                            let mut new_path = path.clone();
                            new_path.push(next);
                            // path.push(next);
                            new_paths.push(new_path);
                            updated = true;
                        }
                    }
                }
            }
        }
        paths = new_paths
            .into_iter()
            .chain(paths.into_iter().filter(|path| {
                let lp = path[path.len() - 1];
                matrix[lp.0 as usize][lp.1 as usize] == 9
            }))
            .collect()
    }
    // println!("{}", paths.len());
    paths.sort_unstable_by_key(|path| path[0]);
    let score_sum: usize = paths
        .iter()
        .chunk_by(|path| path[0])
        .into_iter()
        .map(|(group_key, group)| {
            group
                .filter_map(|path| {
                    let end = path[path.len() - 1];
                    if matrix[end.0 as usize][end.1 as usize] == 9 {
                        Some(end)
                    } else {
                        None
                    }
                })
                .unique()
                .count()
        })
        .sum();
    println!("Part 1 {score_sum}");

    let rating_sum: usize = paths
        .iter()
        .chunk_by(|path| path[0])
        .into_iter()
        .map(|(group_key, group)| {
            group
                .filter_map(|path| {
                    let end = path[path.len() - 1];
                    if matrix[end.0 as usize][end.1 as usize] == 9 {
                        Some(path)
                    } else {
                        None
                    }
                })
                .unique()
                .count()
        })
        .sum();
    println!("Part 2 {rating_sum}");
}

fn to_matrix(data: String) -> Vec<Vec<char>> {
    data.split("\n")
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}
