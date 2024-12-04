use itertools::Itertools;

pub fn day4() {
    // Horrible strategy with 90 degree rotations and diagonals
    day4_part1();
    // Convolution (should really have done this for part 1...)
    day4_part2();
}

fn day4_part1() {
    let data = crate::load_data("data/4").unwrap();
    let char_matrix = to_matrix(data);
    let mut matrix = char_matrix;
    let mut total_count = 0;
    for _transpose_idx in 0..4 {
        let count: usize = count_all_xmas_per_row(&matrix);
        let diag_count = count_all_xmas_per_row(&get_all_diagonals(&matrix));
        total_count += count + diag_count;
        matrix = rot90(&matrix);
    }
    println!("n_xmas: {total_count}");
}

fn day4_part2() {
    let k1 = "
M.M
.A.
S.S"
    .trim();
    let k2 = "
M.S
.A.
M.S"
    .trim();
    let k3 = "
S.M
.A.
S.M"
    .trim();
    let k4 = "
S.S
.A.
M.M"
    .trim();
    let kernels = [k1, k2, k3, k4].map(|k| to_matrix(k.to_string()));
    let data = crate::load_data("data/4").unwrap();
    let matrix = to_matrix(data.to_string());
    let mut count = 0;
    for row in 0..(matrix.len() - 2) {
        for col in 0..(matrix[0].len() - 2) {
            for kernel in &kernels {
                if match_kernel(&matrix, kernel, row, col) {
                    count += 1;
                }
            }
        }
    }
    println!("X-MAS: {count}");
}

pub fn count_xmas(data: &str) -> usize {
    data.chars()
        .collect::<Vec<_>>()
        .windows(4)
        .filter(|window| window.iter().collect::<String>() == "XMAS")
        .count()
}

fn to_matrix(data: String) -> Vec<Vec<char>> {
    data.split("\n")
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn count_all_xmas_per_row(transposed: &Vec<Vec<char>>) -> usize {
    transposed
        .iter()
        .map(|row| count_xmas(&row.iter().collect::<String>()))
        .sum()
}

fn rot90(char_matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..char_matrix[0].len())
        .map(|col_idx| {
            (0..char_matrix.len())
                .map(|row_idx| char_matrix[char_matrix.len() - 1 - row_idx][col_idx])
                .collect_vec()
        })
        .collect_vec()
}

fn get_all_diagonals(char_matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let diags1 = diagonals_from_cols(char_matrix)
        .into_iter()
        .chain(diagonals_from_rows(char_matrix).into_iter());
    diags1.collect()
}

fn diagonals_from_cols(char_matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (1..char_matrix[0].len())
        .map(|col_idx| {
            let mut row = 0;
            let mut col = col_idx;
            let mut diag = Vec::new();
            while col < char_matrix[0].len() && row < char_matrix.len() {
                diag.push(char_matrix[row][col]);
                row += 1;
                col += 1;
            }
            diag
        })
        .collect_vec()
}

fn diagonals_from_rows(char_matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..char_matrix.len())
        .map(|row_idx| {
            let mut row = row_idx;
            let mut col = 0;
            let mut diag = Vec::new();
            while col < char_matrix[0].len() && row < char_matrix.len() {
                diag.push(char_matrix[row][col]);
                row += 1;
                col += 1;
            }
            diag
        })
        .collect_vec()
}

fn match_kernel(
    grid: &Vec<Vec<char>>,
    kernel: &Vec<Vec<char>>,
    row_pos: usize,
    col_pos: usize,
) -> bool {
    for row in 0..3 {
        for col in 0..3 {
            if (row + col) % 2 == 0 {
                if grid[row + row_pos][col + col_pos] != kernel[row][col] {
                    return false;
                }
            }
        }
    }
    return true;
}
