mod utils;

fn part1() {
    const FILE: &str = "./inputs/day1.txt";
    let lines = utils::read_lines_in_file(FILE);

    let mut sum: u32 = 0;
    for line in lines {
        let (first, last) = first_and_last_digits_in(&line);
        sum += 10*first + last;
    }

    println!("Part1: The sum is {sum}");
}

fn part2() {

    const FILE: &str = "./inputs/day1.txt";
    let lines = utils::read_lines_in_file(FILE);
    
    let mut sum: u32 = 0;
    for line in lines {
        let (first, last) = real_first_and_last_digits_in(&line);
        sum += 10*first + last;
    }

    println!("Part2: The real sum is {sum}");

}

fn first_and_last_digits_in(line: &str) -> (u32, u32) {
    let mut result = (0, 0);
    for c in line.chars() {
        if let Some(n) = c.to_digit(10) {
            result.0 = n;
            break;
        }
    }
    for c in line.chars().rev() { 
        if let Some(n) = c.to_digit(10) {
            result.1 = n;
            break;
        }
    }

    result
}

fn real_first_and_last_digits_in(line: &String) -> (u32, u32) {
    let digits: Vec<&str> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digits_str: Vec<&str> = vec!["one1one", "two2two", "three3three", "four4four", "five5five", "six6six", "seven7seven", "eight8eight", "nine9nine"];

    let mut line_ = line.clone();

    for (i, digit) in digits.iter().enumerate() {
        line_ = line_.replace(digit, digits_str[i]);
    }
    //println!("Line {line} and line_ {line_}");

    first_and_last_digits_in(&line_)
}



fn main() {
    part1();
    part2();
}
