use regex::Regex;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_input(filename: &str) -> io::Lines<BufReader<File>> {
    // open the file
    let file = File::open(filename).unwrap();

    // create a buffered reader
    let reader = BufReader::new(file);

    // read the lines into a vector and return it
    reader.lines()
}

fn get_card_number(line: &str) -> u32 {
    let rgx = Regex::new(r"Card\s+(\d+):").unwrap();

    if let Some(captured) = rgx.captures(line) {
        if let Some(card_number_str) = captured.get(1) {
            if let Ok(card_number) = card_number_str.as_str().parse::<u32>() {
                return card_number;
            }
        }
    }

    0
}

fn get_winning_numbers(line: &str) -> HashSet<u32> {
    let mut winning_numbers = HashSet::new();
    let rgx = Regex::new(r"Card\s+\d+: ([\d\s]+)\s+\|").unwrap();

    if let Some(captured) = rgx.captures(line) {
        if let Some(numbers_str) = captured.get(1) {
            let numbers = numbers_str.as_str().split_whitespace();
            for number in numbers {
                // println!("{}", number);
                if let Ok(num) = number.parse::<u32>() {
                    winning_numbers.insert(num);
                }
            }
        }
    }

    winning_numbers
}

fn get_my_numbers(line: &str) -> HashSet<u32> {
    let mut my_numbers = HashSet::new();
    let rgx = Regex::new(r"\| ([\d\s]+)").unwrap();

    if let Some(captured) = rgx.captures(line) {
        if let Some(numbers_str) = captured.get(1) {
            let numbers = numbers_str.as_str().split_whitespace();
            for number in numbers {
                // println!("{}", number);
                if let Ok(num) = number.parse::<u32>() {
                    my_numbers.insert(num);
                }
            }
        }
    }

    my_numbers
}

fn get_num_matches(winning_numbers: &HashSet<u32>, my_numbers: &HashSet<u32>) -> u32 {
    let mut num_matches = 0;

    for num in my_numbers {
        if winning_numbers.contains(num) {
            num_matches += 1;
        }
    }

    num_matches
}

fn count_cards(card_map: &mut HashMap<u32, (u32, u32)>, card_num: u32) {
    // Use entry API to update or insert the value
    let (num_matches, num_cards) = card_map.entry(card_num).or_insert((0, 0));

    // Increment the card count for this card number
    *num_cards += 1;

    for next_card_num in card_num + 1..=card_num + *num_matches {
        // println!("next card: {}", next_card_num);
        count_cards(card_map, next_card_num);
    }
}

fn main() {
    // let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    // let winning_numbers = get_winning_numbers(line);
    // let my_numbers = get_my_numbers(line);
    // dbg!(&winning_numbers);
    // dbg!(&my_numbers);

    let lines = read_input("input.txt");

    // stores the number of matches and number of cards for each card
    let mut card_map: HashMap<u32, (u32, u32)> = HashMap::new();

    for line in lines {
        let card_number = get_card_number(&line.as_ref().unwrap());
        let winning_numbers = get_winning_numbers(&line.as_ref().unwrap());
        let my_numbers = get_my_numbers(&line.as_ref().unwrap());
        let num_matches = get_num_matches(&winning_numbers, &my_numbers);

        card_map.insert(card_number, (num_matches, 0));
    }

    for card_num in 1..=card_map.len() {
        count_cards(&mut card_map, card_num as u32);
    }

    // dbg!(&card_map);

    let total = card_map
        .iter()
        .fold(0, |acc, (_, (_num_matches, num_cards))| acc + num_cards);

    println!("Total cards: {}", total);
}
