mod utils;
use fancy_regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Hand {
    bid: usize,
    strength: usize,
}

impl Hand {
    fn from_line(line: &str, j_is_joker: bool) -> Self {
        let parts: Vec<_> = line.split(' ').collect();
        let cards: String = String::from(parts[0]);
        let bid = parts[1].parse::<usize>().unwrap();

        let mut j_count = 0;
        let mut card_values: HashMap<char, usize> = HashMap::from([
            ( 'A', 13 ), ( 'K', 12 ), ('Q', 11),
            ( 'J', 10 ), ( 'T', 9 ), ( '9', 8 ),
            ( '8', 7 ), ( '7', 6 ), ( '6', 5 ),
            ( '5', 4 ), ( '4', 3 ), ( '3', 2 ),
            ( '2', 1 ),
        ]);

        if j_is_joker { 
            j_count = cards.chars()
                            .filter(|c| *c == 'J')
                            .count();

            card_values = HashMap::from([
                ( 'A', 12 ), ( 'K', 11 ), ('Q', 10),
                ( 'T', 9 ),  ( '9', 8 ),
                ( '8', 7 ),  ( '7', 6 ), ( '6', 5 ),
                ( '5', 4 ),  ( '4', 3 ), ( '3', 2 ),
                ( '2', 1 ),  ( 'J', 0 ),
            ]);
        }

        let hand_type_re: Vec<Regex> = Vec::from([
            Regex::new(r"(?=.*([2-9TJQKA]).*\1)").unwrap(), // One pair
            Regex::new(r"(?=.*([2-9TJQKA]).*\1)(?=.*((?!\1)[2-9TJQKA]).*\2)").unwrap(), // Two pair
            Regex::new(r"(?=.*([2-9TJQKA]).*\1.*\1)").unwrap(), // Three of a kind
            Regex::new(r"(?=.*([2-9TJQKA]).*\1.*\1)(?=.*((?!\1)[2-9TJQKA]).*\2)").unwrap(), // Full house
            Regex::new(r"(?=.*([2-9TJQKA]).*\1.*\1.*\1)").unwrap(), // Four of a kind
            Regex::new(r"(.)\1{4}").unwrap(), // Five of a kind
        ]);
        

        let base_value: usize = cards.chars().collect::<Vec<_>>().iter()
                            .enumerate()
                            .fold(0, |a, (i, b)| a|card_values[b] << ((4-i) * 4));

        let type_values: Vec<_> = hand_type_re.iter()
                                    .enumerate()
                                    .filter_map(|(i, re)| {
                                        if let Ok(did_match) = re.is_match(&cards) {
                                            if did_match { Some(i) } else { None }
                                        } else {
                                            None
                                        }
                                    })
                                    .collect::<Vec<_>>();

        // Get the highest hand type for the current hand or 6 (invalid type) if there is none
        let mut type_value: usize = if let Some(val) = type_values.last().copied() { val } else { 6 };
        if j_is_joker {
            type_value = match type_value {
                5 => type_value,
                4 => match j_count { 1..=5 => 5, _ => type_value },
                3 => match j_count { 1 => 4, 2..=5 => 5, _ => type_value },
                2 => match j_count { 1 => 4, 2 => 5, 3 => 4, _ => type_value },
                1 => match j_count { 1 => 3, 2..=5 => 4, _ => type_value },
                0 => match j_count { 1..=5 => 2, _ => type_value },
                6 => match j_count { 1..=5 => 0, _ => type_value },
                _ => 6, // invalid type
            }
        }
        // Increment type_value for left shifts nd to mark invalid values
        type_value = if type_value == 6 { 0 } else { type_value + 1 };

        //let value: usize = if type_value > 0 { type_value << 20 (1usize << (32usize+type_value)) + base_value } else { base_value };
        let strength: usize = if type_value > 0 { type_value << 20 | base_value } else { base_value };

        Hand {
            bid,
            strength,
        }
    }
}

fn part1() {
    const FILE: &str = "./inputs/day7.txt";
    let lines = utils::read_lines_in_file(FILE);

    let mut hands: Vec<_> = lines.iter()
                            .map(|line| Hand::from_line(line, false))
                            .collect();

    hands.sort_by(|a, b| a.strength.cmp(&b.strength));

    let winnings = hands.iter()
                        .enumerate()
                        .map(|(i, hand)| hand.bid * (i+1))
                        .fold(0, |a, b| a + b);

    println!("Part 1: The total winnings are {winnings}");
}

fn part2() {
    const FILE: &str = "./inputs/day7.txt";
    let lines = utils::read_lines_in_file(FILE);

    let mut hands: Vec<_> = lines.iter()
                            .map(|line| Hand::from_line(line, true))
                            .collect();

    hands.sort_by(|a, b| a.strength.cmp(&b.strength));

    let winnings = hands.iter()
                        .enumerate()
                        .map(|(i, hand)| hand.bid * (i+1))
                        .fold(0, |a, b| a + b);

    println!("Part 2: The total winnings are {winnings}");
}

fn main() {
    part1();
    part2();
}
