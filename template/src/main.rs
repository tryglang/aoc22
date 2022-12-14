use std::{fs, process::exit};

pub mod part1 {
    pub fn solve(input: &str) -> Option<usize> {
        None
    }
}

pub mod part2 {
    pub fn solve(input: &str) -> Option<usize> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "";

    #[test]
        fn test_part1() {

            let answer = part1::solve(INPUT); // Get answer

            let answer = match answer {
                Some(value) => value,
                None => 0,
            };

            assert_eq!(answer, 1234); // test your answer agains test case answer
        }

    #[test]
        fn test_part2() {

            let answer = part2::solve(INPUT); // Get answer

            let answer = match answer {
                Some(value) => value,
                None => 0,
            };

            assert_eq!(answer, 1234); // test your answer agains test case answer
        }
}

fn main() {
    let input = match fs::read_to_string("input.txt") {
        Ok(input) => input,
        Err(_) => match fs::read_to_string("day/input.txt") {
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

