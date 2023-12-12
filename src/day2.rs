mod utils;
use std::cmp;

struct Game {
    id: u32,
    n_red: u32,
    n_green: u32,
    n_blue: u32,
}

impl Game {
    fn is_possible(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        self.n_red <= max_red && self.n_green <= max_green && self.n_blue <= max_blue
    }

    fn power(&self) -> u32 {
        self.n_red * self.n_blue * self.n_green
    }
}

fn part1() {
    const FILE: &str = "./inputs/day2.txt";
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    let lines = utils::read_lines_in_file(FILE);

    let mut sum: u32 = 0;
    
    for line in lines {
        let game = game_from(&line);
        sum += if game.is_possible(MAX_RED, MAX_GREEN, MAX_BLUE) { game.id } else { 0 };
    }

    println!("Part 1: Sum of all possible game IDs is {sum}");
}

fn part2() {
    const FILE: &str = "./inputs/day2.txt";
    let lines = utils::read_lines_in_file(FILE);

    let mut sum: u32 = 0;

    for line in lines {
        let game = game_from(&line);
        sum += game.power();
    }

    println!("Part 2: Sum of all powers of minimum sets is {sum}");
}

fn game_from(line: &str) -> Game {
    let game_and_blocks: Vec<_> = line.split(':').collect();
    let game_id = game_and_blocks[0].split(' ').last().unwrap().parse::<u32>().unwrap();
    let blocks: Vec<_> = game_and_blocks[1].split(';').collect();

    let mut game: Game = Game {
        id: game_id,
        n_red: 0,
        n_green: 0,
        n_blue: 0,
    };

    for block in blocks {
        update_game_with(&block, &mut game);
    }

    game
}

fn update_game_with(block: &str, game: &mut Game) {
    let parts: Vec<_> = block.split(',').collect();

    for color in parts {
        let num_name: Vec<_> = color.trim().split(' ').collect();
        let num: u32 = num_name[0].parse::<u32>().unwrap();
        let name: &str = num_name[1];
        match name {
            "red" => game.n_red = cmp::max(game.n_red, num),
            "green" => game.n_green = cmp::max(game.n_green, num),
            "blue" => game.n_blue = cmp::max(game.n_blue, num),
            _ => (), 
        }
    }
}

fn main() {
    part1();
    part2();
}
