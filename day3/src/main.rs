use regex::Regex;
use std::env;
use std::fs;

const DAY: &str = "day3";

fn main() {
    println!("TEST");
    let test_file_path = format!("/{}/src/{}_test.txt", DAY, DAY);
    let memory_str = read_lines(&test_file_path);

    part1(&memory_str);
    part2(&memory_str);

    println!("FOR REAL");
    let file_path = format!("/{}/src/{}.txt", DAY, DAY);
    let memory_str = read_lines(&file_path);

    part1(&memory_str);
    part2(&memory_str);
}

fn read_lines(filename: &str) -> String {
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);
    println!("Reading {}", abs_filename);
    let contents = fs::read_to_string(abs_filename).expect("Something went wrong reading the file");
    return contents;
}

fn part1(memory_str: &str) {
    let mult: i32 = get_mults(&memory_str);
    println!("part1: {:?}", mult);
}

fn get_mults(memory_str: &str) -> i32 {
    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut mult: i32 = 0;
    for cap in re.captures_iter(&memory_str) {
        let first = &cap[1].parse::<i32>().unwrap();
        let second = &cap[2].parse::<i32>().unwrap();
        mult += first * second;
    }
    return mult;
}

fn part2(memory_str: &str) {
    let dont_regex: Regex = Regex::new(r"don't\(\).*?do\(\)").unwrap();

    let mut xed_memory = format!("{}", memory_str);

    for cap in dont_regex.captures_iter(&memory_str) {
        let cur_capt = &cap.get(0).unwrap();
        let disabled_start = cur_capt.start();
        let disabled_end = cur_capt.end();

        let blanked = vec!['x'; disabled_end - disabled_start];
        let blanked_str: String = blanked.into_iter().collect();
        xed_memory = format!(
            "{}{}{}",
            &xed_memory[..disabled_start],
            blanked_str,
            &xed_memory[disabled_end..]
        );
    }

    let mult: i32 = get_mults(&xed_memory);
    println!("part2: {:?}", mult);
}
