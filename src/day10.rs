mod utils;
use itertools::Itertools;
use std::collections::HashMap;
use std::cmp::min;

fn part1(){
    const FILE: &str = "./inputs/day10.txt";
    let grid = utils::read_chars_in_file(FILE);

    let start: (usize, usize) = find_start(&grid);

    let mut distance: HashMap<(usize, usize), u32> = HashMap::new();
    let mut queue: Vec<((usize, usize), (usize, usize))> = Vec::new();
    distance.insert(start, 0);
    for neighbor in start_neighbors(&start, &grid) {
        queue.push((neighbor, start));
        distance.insert(neighbor, 1);
    }

    while !queue.is_empty() {
        let (current, coming_from) = queue.pop().unwrap();
        let neighbor = neighbor_of(&current, &coming_from, &grid);
        if grid[neighbor.1][neighbor.0] == 'S' {
            continue;
        }
        let dist_from_current = distance[&current] + 1;
        let dist = distance.entry(neighbor).or_insert(dist_from_current);
        *dist = min(*dist, dist_from_current);
        queue.insert(0, (neighbor, current));
    }

    let steps = distance.iter()
        .map(|(_, item)| item)
        .max()
        .unwrap();

    println!("Part 1: The farthest point is {steps} steps away");
}

fn part2(){
    const FILE: &str = "./inputs/day10.txt";
    let grid = utils::read_chars_in_file(FILE);

    let start: (usize, usize) = find_start(&grid);
    let start_neighbors: Vec<(usize, usize)> = start_neighbors(&start, &grid);

    let mut outline: Vec<(usize, usize)> = Vec::new();
    //let mut edges: Vec<(usize, usize)> = Vec::new();
    outline.push(start);
    outline.push(start_neighbors[0]);

    loop {
        let neighbor = neighbor_of(&outline[outline.len()-1], &outline[outline.len()-2], &grid);
        outline.push(neighbor);
        if grid[neighbor.1][neighbor.0] == 'S' {
            break;
        }
    }

    // Shoelace algorithm
    let area = outline.iter().tuple_windows()
                    .map(|(a, b)| (a.0*b.1) as i64 - (a.1*b.0) as i64 )
                    .fold(0, |a, b| a + b)
                    .abs();


    let area = area / 2;

    // Pick's theorem
    // Outline length - 1 b/c we include 'S' twice in the outline
    let area = area - ((outline.len() as i64 - 1) / 2) + 1;

    println!("Part 2: The area within the loop is {area}");
}


fn neighbor_of(pipe: &(usize, usize), coming_from: &(usize, usize), grid: &Vec<Vec<char>>) -> (usize, usize) {
    let pipe = (pipe.0 as i32, pipe.1 as i32);
    let coming_from = (coming_from.0 as i32, coming_from.1 as i32);
    let diff = (pipe.0 - coming_from.0, pipe.1 - coming_from.1);
    let result = match grid[pipe.1 as usize][pipe.0 as usize] {
        '|' => (pipe.0, pipe.1+diff.1),
        '-' => (pipe.0+diff.0, pipe.1),
        'J' => (pipe.0-diff.1, pipe.1-diff.0),
        'L' => (pipe.0+diff.1, pipe.1+diff.0),
        '7' => (pipe.0+diff.1, pipe.1+diff.0),
        'F' => (pipe.0-diff.1, pipe.1-diff.0),
        _ => panic!("Unsupported pipe"),
    };
    (result.0 as usize, result.1 as usize)
}

fn start_neighbors(start: &(usize, usize), grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if start.0 < grid[0].len()-1 && ['-', 'J', '7'].iter().any(|x| x == &grid[start.1][start.0+1]) {
        neighbors.push((start.0+1, start.1));
    }
    if start.0 > 0 && ['-', 'L', 'F'].iter().any(|x| x == &grid[start.1][start.0-1]) {
        neighbors.push((start.0-1, start.1));
    }
    if start.1 < grid.len() - 1 && ['|', 'L', 'J'].iter().any(|x| x == &grid[start.1+1][start.0]) {
        neighbors.push((start.0, start.1+1));
    }
    if start.1 > 0 && ['|', 'F', '7'].iter().any(|x| x == &grid[start.1-1][start.0]) {
        neighbors.push((start.0, start.1-1));
    }
    neighbors
}

fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (row, _) in grid.iter().enumerate() {
        for (col, _) in grid[row].iter().enumerate() {
            if grid[row][col] == 'S' {
                return (col, row);
            }
        }
    }
    panic!("No start pipe found");
}

fn main(){
    part1();
    part2();
}
