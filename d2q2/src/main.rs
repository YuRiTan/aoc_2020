use regex::Regex;
use std::fs::File;
use std::io::Read;

fn is_valid_password(re: &Regex, line: &str) -> bool {
    let caps = re.captures(line).unwrap();
    let first: usize = caps.get(1).unwrap().as_str().parse().unwrap();
    let second: usize = caps.get(2).unwrap().as_str().parse().unwrap();
    let letter: &str = &caps.get(3).unwrap().as_str();
    let password: &str = caps.get(4).unwrap().as_str();

    let check_first = password.chars().nth(first - 1).unwrap_or(' ').to_string() == letter;

    let check_second = password.chars().nth(second - 1).unwrap_or(' ').to_string() == letter;

    let mut result = false;
    if (check_first & !check_second) | (!check_first & check_second) {
        result = true;
    }
    result
}

fn main() {
    let mut f = File::open("../data/day_2_input.txt").unwrap();
    let mut data = String::new();

    f.read_to_string(&mut data).expect("could not read file");

    let numbers: Vec<&str> = data.lines().collect();
    let pattern_matcher = Regex::new(r"^(\d+)-(\d+) ([a-z]{1}): (\w+)$").expect("Invalid regex");

    let result: Vec<bool> = numbers
        .into_iter()
        .map(|x| is_valid_password(&pattern_matcher, x))
        .collect();

    let correct_count: usize = result.iter().filter(|&n| *n).count();
    println!("Number of correct passwords is: {}", correct_count)
}
