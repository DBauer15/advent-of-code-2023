mod utils;

#[derive(Debug)]
struct Race {
    total_length_ms: usize,
    record_distance_mm: usize,
}

fn part1() {
    const FILE: &str = "./inputs/day6.txt";
    let lines = utils::read_lines_in_file(FILE);

    let mut product: usize = 1;
    let races = races_from(&lines, false);
    
    for race in &races {
        for i in 0..race.total_length_ms/2 {
            if traveled_distance_mm(i, race.total_length_ms) > race.record_distance_mm {
                product *= (race.total_length_ms+1) - 2*i;
                break;
            }
        }
    }

    println!("Part 1: The product of all ways to win the races is {product}");
}

fn part2() {
    const FILE: &str = "./inputs/day6.txt";
    let lines = utils::read_lines_in_file(FILE);
    
    let race = races_from(&lines, true);
    let race = &race[0];

    let mut min: usize = 0;
    let mut max: usize = race.total_length_ms / 2;
    let mut mid: usize = (max + min) / 2;

    while min < max {
        let traveled = traveled_distance_mm(mid, race.total_length_ms);

        if traveled < race.record_distance_mm {
            min = mid+1;
            mid = (max + min) / 2;
        } else {
            max = mid-1;
            mid = (max + min) / 2;
        }
    }

    let num_wins: usize = (race.total_length_ms+1) - 2*(mid+1);
    
    println!("Part 2: The number of possible ways to win the race is {num_wins}");
}

fn races_from(lines: &Vec<String>, ignore_space: bool) -> Vec<Race> {
    let times_distances: Vec<Vec<usize>> = lines.iter()
                                                    .map(|line| if ignore_space { 
                                                                        line.replace(" ", "")
                                                                        .split(':')
                                                                        .filter_map(|part| if let Ok(n) = part.trim().parse::<usize>() { 
                                                                            Some(n) 
                                                                        } else { 
                                                                            None })
                                                                        .collect()
                                                                    } else {
                                                                        line.split(' ')
                                                                        .filter_map(|part| if let Ok(n) = part.trim().parse::<usize>() { 
                                                                            Some(n) 
                                                                        } else { 
                                                                            None })
                                                                        .collect()
                                                    })
                                                    .collect();
    times_distances[0].iter()
                    .zip(&times_distances[1])
                    .map(|x| Race {
                        total_length_ms: *x.0,
                        record_distance_mm: *x.1,
                    })
                    .collect()
}

fn traveled_distance_mm(held_ms: usize, total_length_ms: usize) -> usize {
    held_ms * (total_length_ms - held_ms) 
}

fn main() {
    part1();
    part2();
}
