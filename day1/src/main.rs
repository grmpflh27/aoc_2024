use std::env;
use std::fs;

const DAY: &str = "day1";

fn main() {
    println!("TEST");
    let test_file_path = format!("/{}/src/{}_test.txt", DAY, DAY);
    let location_ids = read_lines(&test_file_path);
    let mut left_right = parse_location_ids(&location_ids);

    part1(left_right.clone());
    part2(left_right.clone());

    println!("FOR REAL");
    let file_path = format!("/{}/src/{}.txt", DAY, DAY);
    let location_ids = read_lines(&file_path);
    let mut left_right = parse_location_ids(&location_ids);

    part1(left_right.clone());
    part2(left_right.clone());
}

fn read_lines(filename: &str) -> String {
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);

    println!("Reading {}", abs_filename);
    let contents = fs::read_to_string(abs_filename).expect("Something went wrong reading the file");

    return contents;
}

fn parse_location_ids(location_ids: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in location_ids.lines() {
        let words: Vec<_> = line
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .collect();
        left.push(words[0]);
        right.push(words[1]);
    }
    left.sort();
    right.sort();
    return (left, right);
}

fn part1(left_right: (Vec<i32>, Vec<i32>)) {
    let mut total_distance: i32 = 0;
    for (a, b) in left_right.0.into_iter().zip(left_right.1) {
        total_distance += (a - b).abs();
    }
    println!("part1: {}", total_distance);
}

fn part2(left_right: (Vec<i32>, Vec<i32>)) {
    let mut similarity_score: i32 = 0;
    for cur in left_right.0.into_iter() {
        let match_count = left_right.1.iter().filter(|&i| *i == cur).count() as i32;
        similarity_score += cur * match_count;
    }
    println!("part2: {}", similarity_score);
}
