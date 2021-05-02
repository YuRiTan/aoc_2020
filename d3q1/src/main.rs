use std::fs::File;
use std::io::Read;


fn count_trees(map: Vec<Vec<char>>, start_pos: (usize, usize), slope: (usize, usize)) -> usize {
    let end_row: usize = map.len();
    let map_width: usize = map[0].len();
    let mut pos = start_pos;  // (row, column)
    let mut tree_count: usize = 0;

    while pos.0 < end_row {
        if map[pos.0][pos.1] == '#' {
            tree_count = tree_count + 1
        }
        pos = (pos.0 + slope.0, (pos.1 + slope.1) % map_width)
    }
    tree_count
}

fn main() {
    let mut f = File::open("../data/day_3_input.txt").unwrap();
    let mut data = String::new();

    f.read_to_string(&mut data).expect("could not read file");

    let map: Vec<Vec<char>> = data
        .lines()
        .map(|x| x.chars().into_iter().collect())
        .collect();


    let slope = (1, 3);
    let count: usize = count_trees(map, (0,0), slope);
    println!("Number of trees passed is: {}", count)
}
