use regex::Regex;

use std::collections::HashSet;
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

fn compute_card_value(winning_numbers: &HashSet<u32>, my_numbers: &HashSet<u32>) -> u32 {
    let mut value = 0;

    for num in my_numbers {
        if winning_numbers.contains(num) {
            if value == 0 {
                value = 1;
            } else {
                value *= 2;
            }
        }
    }

    value
}

fn main() {
    // let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    // let winning_numbers = get_winning_numbers(line);
    // let my_numbers = get_my_numbers(line);
    // dbg!(&winning_numbers);
    // dbg!(&my_numbers);
    // dbg!(compute_card_value(&winning_numbers, &my_numbers));

    let lines = read_input("input.txt");
    let total_score = lines.fold(0, |acc, line| {
        let winning_numbers = get_winning_numbers(&line.as_ref().unwrap());
        let my_numbers = get_my_numbers(&line.as_ref().unwrap());
        acc + compute_card_value(&winning_numbers, &my_numbers)
    });

    println!("Total score: {}", total_score);
}
