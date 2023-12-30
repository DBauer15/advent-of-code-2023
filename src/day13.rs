mod utils;
use std::collections::hash_map::DefaultHasher;
use std::hash::{ Hash, Hasher };
use std::cmp::max;

#[derive(Debug)]
struct Pattern {
    row_hashes: Vec<u64>,
    col_hashes: Vec<u64>,
}

impl Pattern {
    fn new(data: &[Vec<char>]) -> Self {
        let row_hashes = data.iter()
                            .map(|row| {
                                let mut hasher = DefaultHasher::new();
                                (*row).hash(&mut hasher);
                                hasher.finish()
                            })
                            .collect();
        let col_hashes = data[0].iter()
                            .enumerate()
                            .map(|(i, _)| {
                                let mut hasher = DefaultHasher::new();
                                data.iter()
                                    .map(|row| row[i])
                                    .collect::<Vec<_>>()
                                    .hash(&mut hasher);
                                hasher.finish()
                            })
                            .collect();
        Pattern {
            row_hashes,
            col_hashes,
        }
    }
}

fn part1() {
    const FILE: &str = "./inputs/day13.txt";
    let data = utils::read_chars_in_file(FILE);
    let mut patterns: Vec<Pattern> = Vec::new();
    let mut raw_patterns: Vec<&[Vec<char>]> = Vec::new();

    let mut start = 0;
    let mut end = 1;
    while end <= data.len() {
        if end == data.len() || data[end].is_empty() {
            let pattern = Pattern::new(&data[start..end]);
            patterns.push(pattern);
            raw_patterns.push(&data[start..end]);
            end += 1;
            start = end;
        }
        end += 1;
    }

    let mut sum = 0;
    for pattern in patterns {
        let cols = find_mirror_position(&pattern.col_hashes); 
        let rows = find_mirror_position(&pattern.row_hashes); 
        sum += cols;
        sum += 100 * rows;
    }

    println!("Part 1: The summary of my notes is {sum}");
}

fn part2() {
}

fn find_mirror_position<T: std::cmp::PartialEq>(sequence: &Vec<T>) -> usize {
    let n: i64 = sequence.len() as i64;

    let mut left = 0;
    let mut length = 1;

    // Start with adjacent items and expand the sequence
    for i in 0..sequence.len() {
        let mut low: i64 = i as i64 - 1;
        let mut high: i64 = i as i64;

        while low >= 0 && high < n && sequence[low as usize] == sequence[high as usize] {

            // Count this palindrome only if it reaches one of the extrema of the sequence
            if low == 0 || high == n-1 {
                left = low;
                length = high - low + 1;
            }
            low -= 1;
            high += 1;
        }
    }

    if left == 0 || left+length == n {
        max(0, left + length/2) as usize
    } else {
        0
    }
}

fn print_grid(grid: &[Vec<char>]) {
    for row in grid {
        println!("{}", String::from_iter(row));
    }
}

fn main() {
    part1();
    part2();
}
