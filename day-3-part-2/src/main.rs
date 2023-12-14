use regex::Regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct PartNumber {
    value: u32,
    local_start: u32,
    len: u32,
    line_num: u32,
    line_size: u32,
}

// gets the locations of all symbols in a line and returns a vector of their positions
fn get_line_symbols(line: &str) -> Vec<u32> {
    let rgx = Regex::new(r"\*").unwrap();
    let mut symbols: Vec<u32> = Vec::new();

    for cap in rgx.captures_iter(line) {
        let index = cap.get(0).unwrap().start();

        // convert from usize to u32
        symbols.push(index.try_into().unwrap());
    }

    symbols
}

fn read_input(filename: &str) -> io::Lines<BufReader<File>> {
    // open the file
    let file = File::open(filename).unwrap();

    // create a buffered reader
    let reader = BufReader::new(file);

    // read the lines into a vector and return it
    reader.lines()
}

fn get_search_locs(num_len: u32, start: u32, line_num: u32, line_size: u32) -> Vec<u32> {
    let mut locs: Vec<u32> = Vec::new();

    // get possible locations of symbols in the line above and below
    // if the number is longer than the line, it can't be in the line above
    if num_len <= line_size {
        // get the start and end positions of the number in the line above

        // if the number starts at the beginning of the line, start at the beginning of the line
        let search_start = if start == 0 { start } else { start - 1 };
        // if the number ends at the end of the line, end at the end of the line
        let search_end = if start + num_len >= line_size {
            line_size
        } else {
            start + num_len + 1
        };

        for i in search_start..search_end {
            // add the positions of the line above to the vector;
            // don't add positions on the line above if we're on
            // the first line (because there is no line above)
            if line_num > 0 {
                locs.push(i + (line_num - 1) * line_size);
            }
            // add the positions of the line below to the vector
            locs.push(i + (line_num + 1) * line_size);
        }

        // add the positions of the current line to the vector;
        // if the number starts at the beginning of the line, we don't need to add the position before it
        if start != 0 {
            locs.push(start - 1 + line_num * line_size);
        }

        // if the number ends at the end of the line, we don't need to add the position after it
        if start + num_len < line_size {
            locs.push(start + num_len + line_num * line_size);
        }
    } else {
        panic!("number is longer than the line somehow");
    }

    locs
}
fn main() {
    // let line = "...$.*....";
    // // let rgx = Regex::new(r"[^.0-9]").unwrap();

    // // for cap in rgx.captures_iter(line) {
    // //     dbg!(&cap.get(0).unwrap().start());
    // // }

    // let symbols = get_line_symbols(line);
    // dbg!(symbols);

    let lines = read_input("input.txt");
    // this stores the absolute positions of all the symbols and the numbers that are adjacent to them
    let mut symbol_set: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut pnum_matches: Vec<PartNumber> = Vec::new();

    let pnum_rgx = Regex::new(r"\d+").unwrap();

    // scan the input file and get the positions of the symbols and the part numbers
    for (idx, line) in lines.enumerate() {
        match line {
            Ok(line) => {
                // get the positions of the symbols within their lines
                let local_symbol_locs = get_line_symbols(&line);

                // compute absolute positions of the symbols
                // let symbol_locs: Vec<u32> = symbols
                //     .iter()
                //     .map(|x| x + (idx * &line.len()) as u32)
                //     .collect();
                // dbg!(symbol_locs);

                // add the absolute positions to the symbol set
                for local_symbol_loc in local_symbol_locs {
                    let loc = local_symbol_loc + (idx * &line.len()) as u32;
                    symbol_set.insert(loc, Vec::new());
                }

                // get all the matches for numbers in the line and add them to pnum_matches
                for cap in pnum_rgx.captures_iter(&line) {
                    let pnum = PartNumber {
                        value: cap.get(0).unwrap().as_str().parse().unwrap(),
                        local_start: cap.get(0).unwrap().start().try_into().unwrap(),
                        len: cap.get(0).unwrap().as_str().len().try_into().unwrap(),
                        line_num: idx.try_into().unwrap(),
                        line_size: line.len().try_into().unwrap(),
                    };

                    // println!("value: {}", pnum.value);
                    // println!("local_start: {}", pnum.local_start);
                    // println!("len: {}", pnum.len);
                    // println!("line_num: {}", pnum.line_num);
                    // println!("line_size: {}", pnum.line_size);
                    // println!();

                    pnum_matches.push(pnum);
                }
            }
            Err(_) => (),
        }
    }

    // dbg!(&symbol_set);

    // dbg!(get_search_locs(2, 8, 2, 10));

    for pnum in pnum_matches {
        // get all the possible locations of a symbol around this part number
        let search_locs =
            get_search_locs(pnum.len, pnum.local_start, pnum.line_num, pnum.line_size);

        for search_loc in search_locs {
            // if this part number is adjacent to a "*", add it to the list of adjacent numbers for that "*"
            if symbol_set.contains_key(&search_loc) {
                // println!("Match: {}", pnum.value);
                let adj_parts = symbol_set.get_mut(&search_loc).unwrap();
                adj_parts.push(pnum.value);
            }
        }
    }

    // find the sum of all of the gears that have exactly 2 adjacent part numbers
    let mut sum = 0;
    for (_key, value) in symbol_set {
        if value.len() == 2 {
            sum += value[0] * value[1];
        }
    }

    println!("Sum: {}", sum);
}
