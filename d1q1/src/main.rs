use std::fs::File;
use std::io::Read;


fn main() {
    let mut f = File::open("../data/day_1_input.txt").unwrap();
    let mut data = String::new();

    f.read_to_string(&mut data).expect("could not read file");

    let numbers: Vec<u32> = data
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let size = numbers.len();
    'outer: for i in 0..size {
        for j in 0..size {
            if numbers[i] + numbers[j] == 2020 {
                println!("found a pair adding up to 2020: {}, {}", numbers[i], numbers[j]);
                println!("the product of these numbers is: {}", numbers[i] * numbers[j]);
                break 'outer;
            }
        }
    }
}
