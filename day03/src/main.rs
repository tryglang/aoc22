use std::{fs, process::exit};

pub mod part1 {
    pub fn solve(input: &str) -> Option<usize> {
        None
    }
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn test_part1() {
            let input = ""; // test case input

            let answer = solve(input); // Get answer

            let answer = match answer {
                Some(value) => value,
                None => 0,
            };

            assert_eq!(answer, 1234); // test your answer agains test case answer
        }
    }
}

pub mod part2 {
    pub fn solve(input: &str) -> Option<usize> {
        None
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn test_part2() {
            let input = ""; // test case input

            let answer = solve(input); // Get answer

            let answer = match answer {
                Some(value) => value,
                None => 0,
            };

            assert_eq!(answer, 1234); // test your answer agains test case answer
        }
    }
}

fn main() {
    let input = match fs::read_to_string("input.txt") {
        Ok(input) => input,
        Err(_) => match fs::read_to_string("day03/input.txt") {
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

