use regex::Regex;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_game_id(game: &str) -> u32 {
    let game_rgx = Regex::new(r"Game (\d+):").unwrap();

    game_rgx
        .captures(game)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}

fn get_sets(game: &str) -> Vec<(u32, u32, u32)> {
    let sets_rgx = Regex::new(r"(\d+) (\w+)([,;]?)").unwrap();
    let mut sets: Vec<(u32, u32, u32)> = Vec::new();

    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;

    // capture indicies are based on this:
    // [src/main.rs:39] &cap = Captures(
    //     {
    //         0: 8..16/"8 green,",
    //         1: 8..9/"8",
    //         2: 10..15/"green",
    //         3: 15..16/",",
    //     },
    // )

    for cap in sets_rgx.captures_iter(game) {
        // dbg!(&cap);
        let color = cap.get(2).unwrap().as_str();
        let number: u32 = cap.get(1).unwrap().as_str().parse().unwrap();

        // get the color
        match color {
            "red" => red = number,
            "green" => green = number,
            "blue" => blue = number,
            _ => panic!(),
        }

        // check for end of set;
        // if we find a comma,
        // we're still in the set (e.g. we run into "8 green,")
        match cap.get(3) {
            Some(cap) => {
                if cap.as_str() != "," {
                    sets.push((red, green, blue));
                    red = 0;
                    green = 0;
                    blue = 0;
                }
            }
            None => (),
        }
    }

    sets
}

fn is_set_possible(candidate: &(u32, u32, u32), expected: &(u32, u32, u32)) -> bool {
    candidate.0 <= expected.0 && candidate.1 <= expected.1 && candidate.2 <= expected.2
}

fn is_game_possible(sets: &Vec<(u32, u32, u32)>, expected: &(u32, u32, u32)) -> bool {
    // dbg!(sets, expected);

    // if any of the sets are not possible, the game is not possible
    for set in sets {
        if !is_set_possible(set, expected) {
            return false;
        }
    }

    true
}

fn read_input(filename: &str) -> io::Lines<BufReader<File>> {
    // open the file
    let file = File::open(filename).unwrap();

    // create a buffered reader
    let reader = BufReader::new(file);

    // read the lines into a vector and return it
    reader.lines()
}

fn main() {
    // let text = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";

    // quick test
    // let sets_rgx = Regex::new(r"(\d+) (\w+)([,;]?)").unwrap();
    // for cap in sets_rgx.captures_iter(text) {
    //     dbg!(cap);
    // }

    // let game_id: u32 = get_game_id(text);
    // let sets = get_sets(text);
    // let is_possible = is_game_possible(&sets, &(12, 13, 14));

    // dbg!(game_id, sets, is_possible);

    // this is how many cubes in the bag we have (red, green, blue)
    let bag = (12, 13, 14);

    // read the input file
    let lines = read_input("input.txt");
    let answer = lines.fold(0, |acc, line| {
        // for each game...
        match line {
            Ok(game) => {
                // get the game id
                let game_id: u32 = get_game_id(&game);
                // get all the sets in the game
                let sets = get_sets(&game);

                // if the game is possible, add the game id to the accumulator
                if is_game_possible(&sets, &bag) {
                    println!("Adding line: {}; Total: {}", &game, acc);
                    acc + game_id
                } else {
                    acc
                }
            }
            Err(_) => acc, // Ignore errors and continue folding
        }
    });

    println!("Answer: {}", answer);
}
