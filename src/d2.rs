use itertools::Itertools;

pub fn day2() {
    let data = crate::load_data("data/2").unwrap();
    let lines = data.split("\n").collect::<Vec<_>>();
    let number_lines = lines
        .iter()
        .map(|line| {
            line.split(" ")
                .map(|d| d.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let safe_reports = number_lines.iter().filter(|report| is_safe(report)).count();
    println!("{}", safe_reports);

    let dampened_safe_reports = number_lines
        .iter()
        .filter(|report| {
            for idx in 0..report.len() {
                let (s1, s2) = report.split_at(idx);
                let new_report = s1.iter().chain(s2.iter().skip(1)).cloned().collect_vec();
                if is_safe(&new_report) {
                    return true;
                }
            }
            false
        })
        .count();
    println!("dampened: {}", dampened_safe_reports);
}

fn is_safe(report: &Vec<i32>) -> bool {
    let diffs = report
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();
    let inc_or_dec = diffs.iter().all(|&x| x > 0) || diffs.iter().all(|&x| x < 0);
    let abs_min_max = diffs.iter().all(|&x| x.abs() >= 1 && x.abs() <= 3);
    inc_or_dec && abs_min_max
}
