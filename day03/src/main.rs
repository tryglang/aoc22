use std::{fs, process::exit};

    fn get_char_value(c: char) -> usize {

        let mut value = 0;
        if c as usize >= 97 && c as usize <= 122 { // lower case
            value = c as usize - 96;
        }

        if c as usize >= 65 && c as usize <= 90 {
            value = c as usize - 38;
        }

        //println!("value: {}", value);
        value
    }

pub mod part1 {
    use crate::get_char_value;

pub fn solve(input: &str) -> Option<usize> {
        let mut sum = 0;
        for line in input.lines() {

            let splitline = line.split_at(line.len() / 2);
            let mut matching_char= None;
            for c in splitline.0.chars() {
                if splitline.1.contains(c) {
                    matching_char = Some(c);
                }
                
            }
            sum += match matching_char {
                Some(value) => get_char_value(value),
                None => 0,
            };

            println!("");
        }
/*
        get_char_value('a');
        println!("{}",get_char_value('L'));
*/
        Some(sum)
    }


    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn test_part1() {
            let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"; // test case input

            let answer = solve(input); // Get answer

            let answer = match answer {
                Some(value) => value,
                None => 0,
            };

            assert_eq!(answer, 157); // test your answer agains test case answer
        }
    }
}

pub mod part2 {
    use crate::get_char_value;

pub fn solve(input: &str) -> Option<usize> {
        let mut result = 0;
        let mut lines = input.lines();

        loop {
            let first = match lines.next(){
                Some(line) => line,
                None => break,
            };

            let second = match lines.next(){
                Some(line) => line,
                None => break,
            };

            let third = match lines.next(){
                Some(line) => line,
                None => break,
            };

            for char in first.chars() {
                if second.contains(char) && third.contains(char) {
                    result += get_char_value(char);
                    break;
                }
            }
        }

        Some(result)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn test_part2() {
            let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"; // test case input

            let answer = solve(input); // Get answer

            let answer = match answer {
                Some(value) => value,
                None => 0,
            };

            assert_eq!(answer, 70); // test your answer agains test case answer
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

