use std::env;
use std::fs;

const DAY: &str = "day4";

fn main() {
    println!("TEST");
    let test_file_path = format!("/{}/src/{}_test.txt", DAY, DAY);

    let word = vec!['X', 'M', 'A', 'S'];
    let word_search = WordSearch::new(&read_lines(&test_file_path), word.clone());
    part1(&word_search);
    part2(&word_search);

    println!("FOR REAL");
    let file_path = format!("/{}/src/{}.txt", DAY, DAY);
    let word_search = WordSearch::new(&read_lines(&file_path), word.clone());
    part1(&word_search);
    part2(&word_search);
}

fn read_lines(filename: &str) -> String {
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);
    let contents = fs::read_to_string(abs_filename).expect("Something went wrong reading the file");
    return contents;
}

#[derive(Debug, Clone, Copy)]
struct Coord {
    y: i32, // row
    x: i32, // col
}

fn get_offsets() -> Vec<Coord> {
    return vec![
        Coord { y: -1, x: 0 },
        Coord { y: -1, x: 1 },
        Coord { y: 0, x: 1 },
        Coord { y: 1, x: 1 },
        Coord { y: 1, x: 0 },
        Coord { y: 1, x: -1 },
        Coord { y: 0, x: -1 },
        Coord { y: -1, x: -1 },
    ];
}

fn get_patch_offsets() -> Vec<Coord> {
    return vec![
        Coord { y: -1, x: -1 },
        Coord { y: -1, x: 0 },
        Coord { y: -1, x: 1 },
        Coord { y: 0, x: -1 },
        Coord { y: 0, x: 0 },
        Coord { y: 0, x: 1 },
        Coord { y: 1, x: -1 },
        Coord { y: 1, x: 0 },
        Coord { y: 1, x: 1 },
    ];
}

#[derive(Debug, Clone, Copy)]
struct WordSearchChar {
    coord: Coord,
    letter: char,
}

#[derive(Debug)]
struct WordSearch {
    _data: Vec<Vec<WordSearchChar>>,
    word: Vec<char>,
}

impl WordSearch {
    fn new(word_search_str: &str, word: Vec<char>) -> Self {
        let mut _data: Vec<Vec<WordSearchChar>> = Vec::new();

        for (i, line) in word_search_str.lines().enumerate() {
            let mut cur_line: Vec<WordSearchChar> = Vec::new();
            for (j, cur) in line.chars().enumerate() {
                cur_line.push(WordSearchChar {
                    coord: Coord {
                        y: i as i32,
                        x: j as i32,
                    },
                    letter: cur,
                });
            }
            _data.push(cur_line);
        }
        return Self { _data, word };
    }

    fn word_size(&self) -> usize {
        return self.word.len();
    }

    fn word_str(&self) -> String {
        return self.word.iter().collect();
    }

    fn rows(&self) -> i32 {
        return self._data.len() as i32;
    }

    fn cols(&self) -> i32 {
        return self._data[0].len() as i32;
    }

    fn in_bounds(&self, coord: Coord) -> bool {
        return coord.y >= 0 && coord.y < self.rows() && coord.x >= 0 && coord.x < self.cols();
    }

    fn at(&self, coord: Coord) -> WordSearchChar {
        return self._data[coord.y as usize][coord.x as usize];
    }

    fn find_all(&self) -> u32 {
        let mut total: u32 = 0;
        for y in 0..self.rows() {
            for x in 0..self.cols() {
                total += self.get_match_count_at(Coord {
                    y: y as i32,
                    x: x as i32,
                });
            }
        }
        return total;
    }

    fn get_match_count_at(&self, coord: Coord) -> u32 {
        let start = self.at(coord);
        let slices = self.get_neighbor_slices(start);

        let mut matches: u32 = 0;

        for slice in slices {
            if slice == self.word_str() {
                matches += 1;
            }
        }

        return matches;
    }

    fn get_neighbor_slices(&self, start: WordSearchChar) -> Vec<String> {
        // println!("getting slices of {:?}", start);
        let mut full_slices: Vec<String> = Vec::new();

        for off in get_offsets() {
            let mut ptr: WordSearchChar = start.clone();
            let mut cur_slice: Vec<char> = vec![ptr.letter];

            for _ in 0..self.word_size() - 1 {
                let next_coord = Coord {
                    y: ptr.coord.y + off.y,
                    x: ptr.coord.x + off.x,
                };
                if !self.in_bounds(next_coord) {
                    break;
                }
                let next = self.at(next_coord);
                cur_slice.push(next.letter);
                ptr = next;
            }
            if cur_slice.len() == self.word_size() {
                let s: String = cur_slice.iter().collect();
                full_slices.push(s);
            }
        }
        return full_slices;
    }

    fn get_patch(&self, start: WordSearchChar) -> Vec<char> {
        let mut patch: Vec<char> = Vec::new();
        for off in get_patch_offsets() {
            let next_coord = Coord {
                y: start.coord.y + off.y,
                x: start.coord.x + off.x,
            };
            if !self.in_bounds(next_coord) {
                break;
            }
            let next = self.at(next_coord);
            patch.push(next.letter);
        }

        if patch.len() != 9 {
            return vec![];
        }
        return patch;
    }
}

fn part1(word_search: &WordSearch) {
    let match_cnt: u32 = word_search.find_all();

    println!("part1: {:?}", match_cnt);
}

fn part2(word_search: &WordSearch) {
    let mut a_coords: Vec<WordSearchChar> = Vec::new();

    for row in word_search._data.clone() {
        for it in row {
            if it.letter == 'A' {
                a_coords.push(it);
            }
        }
    }

    let mut cnt: u32 = 0;
    for a in a_coords {
        let cur_patch = word_search.get_patch(a);
        if cur_patch.is_empty() {
            continue;
        }

        let mut mmss: Vec<char> = vec![cur_patch[0], cur_patch[2], cur_patch[6], cur_patch[8]];
        mmss.sort();
        if mmss == vec!['M', 'M', 'S', 'S'] {
            // eliminate diagonal
            if cur_patch[0] != cur_patch[8] {
                println!("{:?} patch for {:?}", mmss, a);
                cnt += 1;
            }
        }
    }

    println!("part2: {:?}", cnt);
}
