use regex::Regex;
use std::fs::File;
use std::io::Read;

fn is_valid_password(re: &Regex, line: &str) -> bool {
    let caps = re.captures(line).unwrap();
    let min: usize = caps.get(1).unwrap().as_str().parse().unwrap();
    let max: usize = caps.get(2).unwrap().as_str().parse().unwrap();
    let letter: &str = &caps.get(3).unwrap().as_str();
    let password: &str = &caps.get(4).unwrap().as_str();

    let count: usize = password.matches(letter).count();
    return (count >= min) & (count <= max);
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

    let correct_count: usize = result.iter().filter(|&n| *n == true).count();
    println!("Number if correct passwords is: {}", correct_count)
}
