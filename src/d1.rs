pub fn day1() {
    let data = crate::load_data("data/1").unwrap();
    let lines = data.split("\n").collect::<Vec<_>>();
    let mut column1: Vec<_> = lines
        .iter()
        .map(|line| line.split(" ").next().unwrap().parse::<i32>().unwrap())
        .collect();
    let mut column2: Vec<_> = lines
        .iter()
        .map(|line| line.split(" ").last().unwrap().parse::<i32>().unwrap())
        .collect();

    // Part 1
    column1.sort();
    column2.sort();
    let sum: i32 = column1
        .iter()
        .zip(column2.iter())
        .map(|(i1, i2)| (i2 - i1).abs())
        .sum();
    println!("{}", sum);

    // Part 2
    let sim: i32 = column1
        .iter()
        .map(|i1| i1 * column2.iter().filter(|&i2| i2 == i1).count() as i32)
        .sum();
    println!("sim score: {}", sim)
}
