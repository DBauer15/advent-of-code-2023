mod utils;

fn part1() {
    const FILE: &str = "./inputs/day9.txt";
    let lines = utils::read_lines_in_file(FILE);

    let mut sum: i32 = 0;
    for line in lines {
        let series = line.split(' ')
                        .map(|item| item.parse::<i32>().unwrap())
                        .collect();
        let mut derivatives: Vec<Vec<i32>> = Vec::new();
        derivatives.push(series);
        
        let mut current_derivative: &Vec<i32> = derivatives.last().unwrap();
        while current_derivative.iter().any(|item| *item != 0) {
            derivatives.push(derive(current_derivative));
            current_derivative = derivatives.last().unwrap();
        }


        let extrapolated = extrapolate(&derivatives, false);
        sum += extrapolated;
    }

    println!("Part 1: The sum of all extrapolated values is {sum}");
}

fn part2() {
    const FILE: &str = "./inputs/day9.txt";
    let lines = utils::read_lines_in_file(FILE);

    let mut sum: i32 = 0;
    for line in lines {
        let series = line.split(' ')
                        .map(|item| item.parse::<i32>().unwrap())
                        .collect();
        let mut derivatives: Vec<Vec<i32>> = Vec::new();
        derivatives.push(series);
        
        let mut current_derivative: &Vec<i32> = derivatives.last().unwrap();
        while current_derivative.iter().any(|item| *item != 0) {
            derivatives.push(derive(current_derivative));
            current_derivative = derivatives.last().unwrap();
        }

        let extrapolated = extrapolate(&derivatives, true);
        sum += extrapolated;
    }

    println!("Part 2: The sum of all backward extrapolated values is {sum}");
}

fn derive(series: &Vec<i32>) -> Vec<i32> {
    series[..series.len()-1].iter()
                        .enumerate()
                        .map(|(i, _)| series[i+1] - series[i])
                        .collect()
}

fn extrapolate(derivatives: &Vec<Vec<i32>>, backwards: bool) -> i32 {
    derivatives.iter()
                .enumerate()
                .rev()
                .fold(0, |prev_extrap, (i, _)| 
                      match backwards {
                          true => {
                              derivatives[i].first().unwrap() - prev_extrap },
                          false => { 
                              prev_extrap + derivatives[i].last().unwrap()
                          },
                      }
                )
}

fn main() {
    part1();
    part2();
}
