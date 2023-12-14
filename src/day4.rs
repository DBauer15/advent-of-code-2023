mod utils;
use array_tool::vec::Intersect;
use std::collections::HashMap;
use std::cmp::min;

#[derive(Clone)]
struct Card {
    id: usize, 
    common_numbers: Vec<u32>,
}

impl Card {
    fn from_str(line: &str) -> Self {
        let parts: Vec<_> = line.split([':', '|']).collect();
        let id = parts[0].split(' ')
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
        let winning_numbers: Vec<_> = parts[1].trim()
                                                .split(' ')
                                                .filter_map(|num| if let Ok(n) = num.parse::<u32>() { Some(n) } else { None })
                                                .collect();
        let your_numbers: Vec<_> = parts[2].trim()
                                                .split(' ')
                                                .filter_map(|num| if let Ok(n) = num.parse::<u32>() { Some(n) } else { None })
                                                .collect();

        let common_numbers: Vec<_> = winning_numbers.intersect(your_numbers);

        Card {
            id,
            common_numbers,
        }
    }

    fn is_winning(&self) -> bool {
        self.common_numbers.len() > 0
    }

    fn num_matches(&self) -> usize {
        self.common_numbers.len()
    }

    fn points(&self) -> u32 {
        1 << (self.common_numbers.len()-1)
    }
}

fn part1() {
    const FILE: &str = "./inputs/day4.txt";
    let lines = utils::read_lines_in_file(FILE);

    let mut sum: u32 = 0;

    for line in lines {
        let card = Card::from_str(&line);
        if card.is_winning() {
            sum += card.points()
        }
    }

    println!("Part 1: The total number of points is {sum}");
}

fn part2() {
    const FILE: &str = "./inputs/day4.txt";
    let lines = utils::read_lines_in_file(FILE);

    let mut cards: Vec<Card> = Vec::new();

    for line in lines {
        let card = Card::from_str(&line);
        cards.push(card);
    }

    let mut copy_counts = HashMap::new();
    for card in &cards {
        let card_count = copy_counts.entry(card.id).or_insert(0);
        *card_count += 1;

        let cc: u32 = *card_count;

        if card.is_winning() {
            let end = min(card.id+card.num_matches(), cards.len());
            let won_cards = &cards[card.id..end];

            won_cards.iter()
                    .for_each(|won_card| {
                        let won_card_count = copy_counts.entry(won_card.id).or_insert(0);
                        *won_card_count += cc;
                    });
        }
    }

    let sum = copy_counts.iter()
                    .map(|(_, value)| value)
                    .fold(0, |a, b| a+b);

    println!("Part 2: The total number of cards is {sum}");
}


fn main() {
    part1();
    part2();
}
