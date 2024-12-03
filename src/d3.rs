pub fn day3_part1(data: &String) {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum: i32 = 0;
    for capture in re.captures_iter(&data) {
        let d1: i32 = capture[1].parse().unwrap();
        let d2: i32 = capture[2].parse().unwrap();
        sum += d1 * d2;
    }
    println!("Sum of muls: {}", sum);
}

pub fn day3_part2(data: &String) {
    let re = regex::Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();

    #[derive(Eq, PartialEq)]
    enum State {
        Do,
        Dont,
    }
    let mut state = State::Do;
    let mut sum: i32 = 0;
    for caps in re.captures_iter(data) {
        if let Some(matched) = caps.get(0) {
            match matched.as_str() {
                s if s.starts_with("mul") && state == State::Do => {
                    let d1: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
                    let d2: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
                    sum += d2 * d1;
                }
                "do()" => state = State::Do,
                "don't()" => state = State::Dont,
                _ => {}
            }
        }
    }
    println!("Part 2: {}", sum);
}

pub fn day3() {
    let data = crate::load_data("data/3").unwrap();

    day3_part1(&data);
    day3_part2(&data);
}
