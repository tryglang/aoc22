use std::{fs, process::exit};

fn parse_input(input: &str) -> Vec<Vec<char>> {

        let parts: Vec<&str> = input.split("\n\n").collect();

        let mut storage: Vec<Vec<char>> = vec![Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        ];

        for line in parts[0].lines() {
            let mut chars = line.chars();

            let mut index = 0;
            loop {
                chars.next();
                let cargo = chars.next().unwrap();
                if cargo as u8 >= 65 && cargo as u8 <= 90 {
                    storage[index].insert(0, cargo);
                }
                chars.next();

                match chars.next() {
                    None => {break},
                    _ => {},
                }

                index += 1;
            }

        }
        storage
}

pub mod part1 {
    use crate::parse_input;

    pub fn solve(input: &str) -> Option<String> {


        let parts: Vec<&str> = input.split("\n\n").collect();

        let mut storage = parse_input(input);

        let mut solution: Vec<char> = Vec::new();

        for line in parts[1].lines() {
            let split_line: Vec<&str> = line.split(" ").collect();

            let amount = split_line[1].parse::<usize>().unwrap();
            let from = split_line[3].parse::<usize>().unwrap();
            let to = split_line[5].parse::<usize>().unwrap();

            for _ in 0..amount {

                let temp = storage[from - 1].pop().unwrap();
                storage[to -1].push(temp);
            }
            
        }

        for mut i in storage {
            match i.pop() {
                Some(value) => {solution.push(value)},
                _ => {},
            }
        }

        let solution = solution.iter().collect::<String>();
        Some(solution)
    }
}

pub mod part2 {
    use crate::parse_input;

pub fn solve(input: &str) -> Option<String> {

        let parts: Vec<&str> = input.split("\n\n").collect();

        let mut storage = parse_input(input);

        let mut solution: Vec<char> = Vec::new();

        for line in parts[1].lines() {
            let split_line: Vec<&str> = line.split(" ").collect();

            let amount = split_line[1].parse::<usize>().unwrap();
            let from = split_line[3].parse::<usize>().unwrap();
            let to = split_line[5].parse::<usize>().unwrap();

                let unit_length = storage[to - 1].len();
            for _ in 0..amount {
                let temp = storage[from - 1].pop().unwrap();
                storage[to -1].insert(unit_length, temp);
            }
            
        }

        for mut i in storage {
            match i.pop() {
                Some(value) => {solution.push(value)},
                _ => {},
            }
        }

        let solution = solution.iter().collect::<String>();
        Some(solution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
        fn test_part1() {

            let answer = part1::solve(INPUT); // Get answer

            let answer = match answer {
                Some(value) => value,
                None => String::from(""),
            };

            assert_eq!(answer, "CMZ"); // test your answer agains test case answer
        }

    #[test]
        fn test_part2() {

            let answer = part2::solve(INPUT); // Get answer

            let answer = match answer {
                Some(value) => value,
                None => String::from(""),
            };

            assert_eq!(answer, "MCD"); // test your answer agains test case answer
        }
}

fn main() {
    let input = match fs::read_to_string("input.txt") {
        Ok(input) => input,
        Err(_) => match fs::read_to_string("day05/input.txt") {
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

