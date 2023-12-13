mod utils;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Part {
    position: (usize, usize),
    symbol: char,
}

impl Part {
    fn is_gear(&self) -> bool {
        self.symbol == '*'
    }
}

fn part1() {
    const FILE: &str = "./inputs/day3.txt";
    let schematic = utils::read_chars_in_file(FILE);

    let mut sum: u32 = 0;

    for row in 0..schematic.len() {
        let mut col = 0;
        while col < schematic[row].len() {
            if schematic[row][col].is_numeric() {
                let len = len_of(&schematic, row, col);
                if let Some(_) = find_adjacent_part(&schematic, row, col, len) {
                    let part_number = (schematic[row][col..col+len]).iter().collect::<String>().parse::<u32>().unwrap();
                    sum+=part_number;
                }
                col+=len;
            } else {
                col+=1;
            }
        }
    }

    println!("Part 1: The sum of part numbers is {sum}");
}

fn part2() {
    let FILE: &str = "./inputs/day3.txt";
    let schematic = utils::read_chars_in_file(FILE);

    let mut sum: u32 = 0;
    let mut gears = HashMap::new();

    for row in 0..schematic.len() {
        let mut col = 0;
        while col < schematic[row].len() {
            if schematic[row][col].is_numeric() {
                let len = len_of(&schematic, row, col);
                if let Some(part) = find_adjacent_part(&schematic, row, col, len) {
                    if part.is_gear() {
                        let part_number = (schematic[row][col..col+len]).iter().collect::<String>().parse::<u32>().unwrap();
                        let count = gears.entry(part).or_insert((0, 1));
                        (*count).0 += 1;
                        (*count).1 *= part_number;
                    }
                }
                col+=len;
            } else {
                col+=1;
            }
        }
    }

    for (_, value) in gears.iter() {
        if value.0 == 2 {
            sum += value.1;
        }
    }

    println!("Part 2: The sum of gear ratios is {sum}");
}

fn len_of(schematic: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let mut col_ptr: usize = col+1;
    while col_ptr < schematic[row].len() && schematic[row][col_ptr].is_numeric() {
        col_ptr+=1;
    }
    col_ptr - col
}

fn find_adjacent_part(schematic: &Vec<Vec<char>>, row: usize, col: usize, len: usize) -> Option<Part> {
    for col_ptr in col..col+len {
        for u in 0..3 {
            for v in 0..3 {
                if row+u > 0 && row+u <= schematic.len() && col_ptr+v > 0 && col_ptr+v <= schematic[row].len() {
                    let row_idx = row+u-1;
                    let col_idx = col_ptr+v-1;
                    if !schematic[row_idx][col_idx].is_numeric() && schematic[row_idx][col_idx] != '.' {
                        let part: Part = Part {
                            position: (row_idx, col_idx),
                            symbol: schematic[row_idx][col_idx],
                        };
                        return Some(part);
                    }
                }
            }
        }
    }

    None
}

fn main() {
    part1();
    part2();
}
