use std::{fs, process::exit};

pub mod part1 {
    pub fn solve(input: &str) -> Option<usize> {

        let mut result = 0;

        for line in input.lines() {
            let split: Vec<&str> = line.split(" ").collect();
            let opponent = split[0];
            let me = split[1];

            match me {
                "X" => {match opponent {
                    "A" => result += 4,
                    "B" => result += 1,
                    "C" => result += 7,
                    _ => {},
                }},
                "Y" => match opponent {
                    "A" => result += 8,
                    "B" => result += 5,
                    "C" => result += 2,
                    _ => {},
                },
                "Z" => match opponent {
                    "A" => result += 3,
                    "B" => result += 9,
                    "C" => result += 6,
                    _ => {}
                },
                _ => {},

            }
        }

        Some(result)
    }
}

pub mod part2 {
    pub fn solve(input: &str) -> Option<usize> {

        let mut score = 0;

        for line in input.lines() {
            let split: Vec<&str> = line.split(" ").collect();
            let opponent = split[0];
            let result = split[1];

            match result {
                "X" => {match opponent {
                    "A" => score += 3,
                    "B" => score += 1,
                    "C" => score += 2,
                    _ => {},
                }},
                "Y" => match opponent {
                    "A" => score += 4,
                    "B" => score += 5,
                    "C" => score += 6,
                    _ => {},
                },
                "Z" => match opponent {
                    "A" => score += 8,
                    "B" => score += 9,
                    "C" => score += 7,
                    _ => {}
                },
                _ => {},

            }
        }
        Some(score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "A Y
B X
C Z"; 

    #[test]
        fn test_part1() {

            let answer = part1::solve(INPUT); // Get answer

            let answer = match answer {
                Some(value) => value,
                None => 0,
            };

            assert_eq!(answer, 15); // test your answer agains test case answer
        }

    #[test]
        fn test_part2() {

            let answer = part2::solve(INPUT); // Get answer

            let answer = match answer {
                Some(value) => value,
                None => 0,
            };

            assert_eq!(answer, 12); // test your answer agains test case answer
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

