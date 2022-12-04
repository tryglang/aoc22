use std::{fs, process::exit};

pub mod part1 {
    pub fn solve(input: &str) -> Option<usize> {
        let data = input.split("\n");
        for i in data {
            println!("{:?}", i);
        }
        None
    }
}

pub mod part2 {
    pub fn solve(input: &str) -> Option<usize> {
        None
    }
}

fn main() {
    let input = match fs::read_to_string("input.txt") {
        Ok(input) => input,
        Err(_) => match fs::read_to_string("day02/input.txt") {
            Ok(input) => input,
            Err(_) => {
                println!("Could not find input file");
                exit(1);
            }
        }
    };


    match part1::solve(&input) {
        Some(input) => println!("Part 1: {}", input),
        None => println!("Part 1: Unfinished"),
    }

    match part2::solve(&input) {
        Some(input) => println!("Part 2: {}", input),
        None => println!("Part 2: Unfinished"),
    }
}

