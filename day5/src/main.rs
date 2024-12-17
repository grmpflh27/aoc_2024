use std::collections::HashMap;
use std::env;
use std::fs;

const DAY: &str = "day5";
const MISSING_TRUE: bool = true;

#[derive(Debug, Clone)]
struct Node {
    page: i32,
    pages_after: Vec<i32>,
}

#[derive(Debug, Clone)]
struct SleighLaunchSafetyManual {
    page_order_rules: HashMap<i32, Node>,
    page_number_updates: Vec<Vec<i32>>,
}

impl SleighLaunchSafetyManual {
    fn page_updates_right_order(&self, page_updates: &Vec<i32>) -> bool {
        for pos in 0..page_updates.len() {
            let result: (bool, i32) = self._check_single(page_updates, pos);
            if !result.0 {
                return false;
            }
        }
        return true;
    }

    fn _check_single(&self, page_updates: &Vec<i32>, pos: usize) -> (bool, i32) {
        let cur: i32 = page_updates[pos];

        if !self.page_order_rules.contains_key(&cur) {
            return (MISSING_TRUE, -1);
        }

        let pre = &page_updates[..pos];
        let post = &page_updates[pos + 1..];
        let node: &Node = self.page_order_rules.get(&cur).unwrap();

        for cur_pre in pre.iter() {
            if node.pages_after.contains(&cur_pre) {
                return (false, *cur_pre);
            }
        }
        for cur_post in post.iter() {
            if !node.pages_after.contains(&cur_post) {
                return (false, *cur_post);
            }
        }
        return (true, 1);
    }

    fn page_updates_problem(&self, page_updates: &Vec<i32>) {
        for pos in 0..page_updates.len() {
            let result: (bool, i32) = self._check_single(page_updates, pos);
            if !result.0 {
                println!("{} is problem of {:?}", result.1, page_updates);
            }
        }
    }
}

fn main() {
    println!("TEST");
    let test_file_path = format!("/{}/src/{}_test.txt", DAY, DAY);

    let instrs_str = &read_lines(&test_file_path);
    let manual = parse(&instrs_str);

    part1(&manual);
    part2(&manual);

    println!("FOR REAL");
    let file_path = format!("/{}/src/{}.txt", DAY, DAY);
    let instrs_str = &read_lines(&file_path);
    let manual = parse(&instrs_str);
    part1(&manual);
    part2(&manual);
}

fn read_lines(filename: &str) -> String {
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);
    let contents = fs::read_to_string(abs_filename).expect("Something went wrong reading the file");
    return contents;
}

fn parse(instrs_str: &String) -> SleighLaunchSafetyManual {
    let empty: Vec<_> = instrs_str
        .lines()
        .enumerate()
        .filter(|(i, l)| l.is_empty())
        .collect();

    let split_at: usize = empty[0].0;
    let lines: Vec<_> = instrs_str.lines().collect();

    let mut page_order_rules: HashMap<i32, Node> = HashMap::new();
    for line in &lines[..split_at] {
        let instr: Vec<_> = line.split('|').map(|i| i.parse::<i32>().unwrap()).collect();
        let mut node: Node = Node {
            page: instr[0],
            pages_after: vec![instr[1]],
        };
        if !page_order_rules.contains_key(&node.page) {
            page_order_rules.insert(node.page, node);
        } else {
            let mut existing = &page_order_rules[&node.page];
            node.pages_after.extend(existing.pages_after.clone());
            page_order_rules.insert(node.page, node);
        }
    }

    let mut page_number_updates: Vec<Vec<i32>> = Vec::new();
    for line in &lines[split_at + 1..] {
        let page: Vec<_> = line.split(',').map(|i| i.parse::<i32>().unwrap()).collect();
        page_number_updates.push(page);
    }
    return SleighLaunchSafetyManual {
        page_order_rules: page_order_rules,
        page_number_updates: page_number_updates,
    };
}

fn part1(manual: &SleighLaunchSafetyManual) {
    let mut middle_page_acc: i32 = 0;
    for page_updates in manual.page_number_updates.iter() {
        if manual.page_updates_right_order(&page_updates) {
            // get middl&e
            middle_page_acc += page_updates[page_updates.len() / 2 as usize];
        }
    }
    println!("part1: {:?}", middle_page_acc);
}

fn part2(manual: &SleighLaunchSafetyManual) {
    let mut invalid_ordered: Vec<&Vec<i32>> = Vec::new();
    for page_updates in manual.page_number_updates.iter() {
        if !manual.page_updates_right_order(&page_updates) {
            invalid_ordered.push(page_updates);
        }
    }

    for invalid in invalid_ordered {
        manual.page_updates_problem(&invalid);
    }
    println!("part1: {:?}", 0);
}
