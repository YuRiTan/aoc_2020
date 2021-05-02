use std::fs::File;
use std::io::Read;
use itertools::Itertools;


fn main() {
    let mut f = File::open("../data/day_1_input.txt").unwrap();
    let mut data = String::new();

    f.read_to_string(&mut data).expect("could not read file");

    let numbers: Vec<u32> = data
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    for triplet in numbers.into_iter().combinations(3) {
        let sum: u32 = triplet.iter().sum();
        if sum == 2020 {
            let product: u32 = triplet.iter().product();
            println!("Found triplet adding up to 2020: {}", sum);
            println!("The product of the triplet is: {}", product);
        }
    }
}
