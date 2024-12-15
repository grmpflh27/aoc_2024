use std::env;
use std::fs;

const DAY: &str = "day2";

fn main() {
    println!("TEST");
    let test_file_path = format!("/{}/src/{}_test.txt", DAY, DAY);
    let reports_str = read_lines(&test_file_path);
    let reports = parse_reports(&reports_str);

    let save_size = part1(reports.clone());
    part2(reports.clone(), save_size);

    println!("FOR REAL");
    let file_path = format!("/{}/src/{}.txt", DAY, DAY);
    let reports_str = read_lines(&file_path);
    let reports = parse_reports(&reports_str);

    let save_size = part1(reports.clone());
    part2(reports.clone(), save_size);
}

fn read_lines(filename: &str) -> String {
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);

    println!("Reading {}", abs_filename);
    let contents = fs::read_to_string(abs_filename).expect("Something went wrong reading the file");

    return contents;
}

fn parse_reports(reports_str: &str) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in reports_str.lines() {
        let level: Vec<_> = line
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .collect();
        reports.push(level);
    }
    return reports;
}

fn part1(reports: Vec<Vec<i32>>) -> usize {
    let save_size: usize = reports
        .into_iter()
        .map(|r| is_save_part1(&r))
        .filter(|s| *s == true)
        .count();
    println!("part1: {:?}", save_size);
    return save_size;
}

fn is_save_part1(levels: &Vec<i32>) -> bool {
    let is_asc: bool = levels[0] - levels[1] < 0;

    let mut prior: i32 = levels[0];
    for (idx, cur) in levels[1..].iter().enumerate() {
        if !is_pair_save(&prior, cur, is_asc) {
            return false;
        }
        prior = levels[1..][idx];
    }

    return true;
}

fn is_pair_save(first: &i32, second: &i32, is_asc: bool) -> bool {
    let diff = second - first;
    if diff.abs() > 3 || diff.abs() < 1 {
        return false;
    }

    if is_asc && diff < 0 {
        return false;
    }

    if !is_asc && diff > 0 {
        return false;
    }
    return true;
}

fn part2(reports: Vec<Vec<i32>>, save_size: usize) {
    // retry all the unsafe ones with problem dampener
    let unsave: Vec<_> = reports.iter().filter(|r| !is_save_part1(r)).collect();

    let dampener_save_size: usize = unsave
        .iter()
        .map(|r| apply_problem_dampener(&r))
        .filter(|s| *s == true)
        .count();
    println!("part2: {:?}", dampener_save_size + save_size);
}

fn apply_problem_dampener(levels: &Vec<i32>) -> bool {
    for dropIdx in 0..levels.len() {
        let mut dampened_levels = levels.clone();
        dampened_levels.remove(dropIdx);
        let cur_is_save = is_save_part1(&dampened_levels);
        if cur_is_save {
            return true;
        }
    }
    return false;
}
