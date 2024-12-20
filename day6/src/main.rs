use std::collections::HashSet;
use std::env;
use std::fs;

const DAY: &str = "day6";

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    y: i32, // row
    x: i32, // col
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Debug, Clone)]
struct Guard {
    position: Coord,
    direction: Direction,
    visited: HashSet<Coord>,
    visited_obstacles: HashSet<(Coord, Direction)>,
}

impl Guard {
    fn rotate(&mut self) {
        self.direction = match self.direction {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }

    fn record_step(&mut self) {
        self.visited.insert(self.position);
    }

    fn record_obstacle_direction(&mut self) {
        self.visited_obstacles
            .insert((self.position, self.direction));
    }

    fn _get_offset(&self) -> Coord {
        let offset: Coord = match self.direction {
            Direction::UP => Coord { y: -1, x: 0 },
            Direction::RIGHT => Coord { y: 0, x: 1 },
            Direction::DOWN => Coord { y: 1, x: 0 },
            Direction::LEFT => Coord { y: 0, x: -1 },
        };
        return offset;
    }

    fn get_next_coord(&self) -> Coord {
        let off = self._get_offset();
        return Coord {
            y: self.position.y + off.y,
            x: self.position.x + off.x,
        };
    }
}

#[derive(Debug)]
struct Board {
    guard: Guard,
    obstacles: Vec<Coord>,
    rows: i32,
    cols: i32,
}

impl Board {
    fn new(board_str: &String) -> Self {
        let mut obstacles: Vec<Coord> = Vec::new();
        let mut guard_coord = Coord { y: 0, x: 0 };

        let lines: Vec<_> = board_str.split('\n').collect();
        for (i, line) in lines.iter().enumerate() {
            for (j, cur) in line.chars().enumerate() {
                if cur == '#' {
                    obstacles.push(Coord {
                        y: i as i32,
                        x: j as i32,
                    });
                }

                if cur == '^' {
                    guard_coord = Coord {
                        y: i as i32,
                        x: j as i32,
                    }
                }
            }
        }

        let first_line_chars: Vec<_> = lines[0].chars().collect();
        return Self {
            guard: Guard {
                position: guard_coord,
                direction: Direction::UP,
                visited: HashSet::new(),
                visited_obstacles: HashSet::new(),
            },
            obstacles: obstacles,
            rows: lines.len() as i32,
            cols: first_line_chars.len() as i32,
        };
    }

    fn move_guard(&mut self) -> bool {
        self.guard.record_step();
        let mut next_coord = self.guard.get_next_coord();
        // println!(
        //     "moving guard from {:?} to {:?}",
        //     self.guard.position, next_coord
        // );
        if self.obstacles.contains(&next_coord) {
            // abort if obstacle <position, direction> already seen
            let seen = (self.guard.position, self.guard.direction);
            if self.guard.visited_obstacles.contains(&seen) {
                return true;
            }

            self.guard.record_obstacle_direction();
            self.guard.rotate();
            next_coord = self.guard.get_next_coord();
            // println!(
            //     "OBSTACLE: moving guard from {:?} to {:?}",
            //     self.guard.position, next_coord
            // );
        }
        self.guard.position = next_coord;
        return false;
    }

    fn simulate(&mut self) -> bool {
        while self.guard.position.y >= 0
            && self.guard.position.y < self.rows
            && self.guard.position.x >= 0
            && self.guard.position.x < self.cols
        {
            let is_stuck = self.move_guard();
            if is_stuck {
                println!("stuck at {:?}", self.guard.position);
                return true;
            }
        }
        return false;
    }

    fn simulate_with_loop_check(&mut self) {
        while self.guard.position.y >= 0
            && self.guard.position.y < self.rows
            && self.guard.position.x >= 0
            && self.guard.position.x < self.cols
        {
            self.move_guard();
        }
    }
}

fn read_lines(filename: &str) -> String {
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);
    let contents = fs::read_to_string(abs_filename).expect("Something went wrong reading the file");
    return contents;
}

fn main() {
    println!("TEST");
    let test_file_path = format!("/{}/src/{}_test.txt", DAY, DAY);

    let board_str = &read_lines(&test_file_path);
    let mut board = Board::new(&board_str);

    part1(&mut board);

    let mut board = Board::new(&board_str);
    part2(&mut board);

    println!("FOR REAL");
    let file_path = format!("/{}/src/{}.txt", DAY, DAY);

    let board_str = &read_lines(&file_path);
    let mut board = Board::new(&board_str);
    part1(&mut board);

    let mut board = Board::new(&board_str);
    part2(&mut board);
}

fn part1(board: &mut Board) {
    board.simulate();
    println!("part1: {:?}", board.guard.visited.len());
}

fn part2(board: &mut Board) {
    // brute force by placing stone in every position and checking for being stuck
    let guard_init = board.guard.clone();

    let mut stuck_cnt = 0;

    for stone_y in 0..board.rows {
        for stone_x in 0..board.cols {
            let stone_coord: Coord = Coord {
                y: stone_y,
                x: stone_x,
            };

            if stone_coord == board.guard.position {
                continue;
            }
            board.obstacles.push(stone_coord);

            let is_stuck = board.simulate();
            if is_stuck {
                stuck_cnt += 1;
            }

            board.obstacles.pop();
            board.guard = guard_init.clone();
        }
    }

    // 2124 too low
    println!("part2: {:?}", stuck_cnt);
}
