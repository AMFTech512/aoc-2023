use regex::Regex;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn parse_digit(digit: &str) -> u32 {
    match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        _ => panic!("Invalid digit"),
    }
}

fn compute_calibration(input: &str) -> u32 {
    // create a regex to capture the digits
    let rgx1 = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let rgx2 = Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();

    // get the first and last digits
    let digit1 = parse_digit(rgx1.find(input).unwrap().as_str());
    let digit2 = parse_digit(
        rgx2.find(input.chars().rev().collect::<String>().as_str())
            .unwrap()
            .as_str()
            .chars()
            .rev()
            .collect::<String>()
            .as_str(),
    );

    // print the vector
    // dbg!(&digits);

    let calibration = digit1 * 10 + digit2;
    calibration
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
    // this is a test for overlapping letters...
    // notice that at the end, there is "eightwo", where "eight" and "two" overlap.
    // this screws up normal regex matching, so we need to use a regex that matches
    // the digits in reverse order.
    let test1 = compute_calibration("nrzpqk3fivesldclpcbfmdtbbhpxonethreeeightwor");
    println!("test1: {}", test1);

    let lines = read_input("input.txt");
    let sum = lines.fold(0, |acc, x| acc + compute_calibration(x.unwrap().as_str()));
    println!("Sum: {}", sum);
}
