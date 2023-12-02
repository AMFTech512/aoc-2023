use regex::Regex;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn compute_calibration(input: &str) -> u32 {
    // create a regex to capture the digits
    let rgx = Regex::new(r"\d").unwrap();

    // extract the digits into a vector
    let digits: Vec<u32> = rgx
        .captures_iter(input)
        .map(|mat| 
            // get the first item in the capture group
            mat.get(0).unwrap()
            // convert it into a string slice
            .as_str()
            // convert the string slice into a u32
            .parse::<u32>().unwrap())
        // collect the iterator into a vector
        .collect();

    // print the vector
    // dbg!(&digits);

    let calibration = digits.first().unwrap() * 10 + digits.last().unwrap();
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

    let lines = read_input("input.txt");
    let sum = lines.fold(0, |acc, x| acc + compute_calibration(x.unwrap().as_str()));
    println!("Sum: {}", sum);
}
