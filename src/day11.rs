mod utils;

fn part1() {
    const FILE: &str = "./inputs/day11.txt";
    let map = utils::read_chars_in_file(FILE);
    let mut galaxies = find_galaxies(&map);
    expand_galaxies(&mut galaxies, &map, 2);
    let distances = find_pairwise_distances(&galaxies);

    let sum: i64 = distances.iter().sum();
    println!("Part 1: The sum of all shortest distances is {sum}");
}

fn part2() {
    const FILE: &str = "./inputs/day11.txt";
    let map = utils::read_chars_in_file(FILE);
    let mut galaxies = find_galaxies(&map);
    expand_galaxies(&mut galaxies, &map, 1000000);
    let distances = find_pairwise_distances(&galaxies);

    let sum: i64 = distances.iter().sum();
    println!("Part 2: The sum of all shortest distances is {sum}");
}

fn find_pairwise_distances(coords: &Vec<(i64, i64)>) -> Vec<i64> {

    let mut distances: Vec<i64> = Vec::new();
    for i in 0..coords.len()-1 {
        for j in i+1..coords.len() {
            let p0: &(i64, i64) = &coords[i];
            let p1: &(i64, i64) = &coords[j];
            distances.push((p0.0 - p1.0).abs() + (p0.1 - p1.1).abs());
        }
    }

    distances
}

fn find_galaxies(map: &Vec<Vec<char>>) -> Vec<(i64, i64)> {
    let mut galaxies: Vec<(i64, i64)> = Vec::new();

    for (i, row) in map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col == &'#' {
                galaxies.push((j as i64, i as i64));
            }
        }
    }
    
    galaxies
}

fn expand_galaxies(coords: &mut Vec<(i64, i64)>, map: &Vec<Vec<char>>, expansion_factor: i64) {
    let mut row_expansions: Vec<i64> = Vec::new();
    let mut curr_expansion: i64 = 0;
    for row in map.iter() {
        if row.iter().all(|entry| entry == &'.') {
            curr_expansion += expansion_factor - 1;
        }
        row_expansions.push(curr_expansion);
    }

    let mut col_expansions: Vec<i64> = Vec::new();
    let mut curr_expansion: i64 = 0;
    for (i, _) in map[0].iter().enumerate() {
        if map.iter().all(|row| row[i] == '.') {
            curr_expansion += expansion_factor - 1;
        }
        col_expansions.push(curr_expansion);
    }

    for coord in coords {
        coord.0 += col_expansions[coord.0 as usize];
        coord.1 += row_expansions[coord.1 as usize];
    }
}

fn main() {
    part1();
    part2();
}
