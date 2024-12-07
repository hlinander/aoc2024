use itertools::Itertools;

pub fn day7() {
    get_test_value_sum_for_ops(vec![|a: i64, b: i64| a * b, |a: i64, b: i64| a + b]);
    get_test_value_sum_for_ops(vec![
        |a: i64, b: i64| a * b,
        |a: i64, b: i64| a + b,
        |a: i64, b: i64| format!("{a}{b}").parse::<i64>().unwrap(),
    ]);
}

pub fn get_test_value_sum_for_ops(ops: Vec<fn(i64, i64) -> i64>) {
    let data = crate::load_data("data/7").unwrap();
    let lines = data.split("\n").collect_vec();
    let mut sum = 0;
    for line in &lines {
        let (res, numbers) = line.split_once(":").unwrap();
        let res = res.parse::<i64>().unwrap();
        let numbers = numbers
            .trim()
            .split(" ")
            .map(|n| n.parse::<i64>().unwrap())
            .collect_vec();
        let mut accums: Vec<i64> = Vec::new();
        for num in &numbers {
            accums = accums
                .into_iter()
                .map(|accum| ops.iter().map(move |op| op(accum, *num)))
                .flatten()
                .collect();
            if accums.len() == 0 {
                accums.push(*num);
            }
        }
        if accums.iter().any(|&accum| accum == res) {
            sum += res;
        }
    }
    println!("sum {}", sum);
}
