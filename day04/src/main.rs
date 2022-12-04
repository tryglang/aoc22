use std::{fs, process::exit};

pub mod part1 {
    pub fn solve(input: &str) -> Option<usize> {
        let lines = input.split("\n");



        let mut t_overlap = 0;
        for i in lines {
            let pair = i.split(",");

            let e1;//pair.next().unwrap();
            let e2;
            let mut temp = Vec::new();
            for i in pair {
                temp.push(i);
            }
            e1 = temp[0];
            e2 = temp[1];
            let range1 = e1.split("-");
            let mut range_a1 = Vec::new();
            for j in range1 {
                range_a1.push(j);
            }

            let mut el1 = 0;
            let mut eh1 = 0;
            if range_a1.len() == 2 {
                el1 = range_a1[0].parse::<usize>().unwrap();//range1.next().unwrap();
                eh1 = range_a1[1].parse::<usize>().unwrap();//range1.next().unwrap();
            }

            //let e2 = pair.next().unwrap();
            let range2 = e2.split("-");
            let mut range_a2 = Vec::new();
            for j in range2 {
                range_a2.push(j);
            }

            let mut el2 = 0;
            let mut eh2 = 0;
            if range_a2.len() == 2 {

                el2 = range_a2[0].parse::<usize>().unwrap();//range1.next().unwrap();
                eh2 = range_a2[1].parse::<usize>().unwrap();//range1.next().unwrap();
            }
            //let el2 = range2.next().unwrap();
            //let eh2 = range2.next().unwrap();

                //println!("range 1: {}, {}. range 2: {}, {}", el1, eh1, el2, eh2);
            if (el1 >= el2 && eh1 <= eh2) || (el1 <= el2 && eh1 >= eh2) {
                t_overlap += 1;
            }
        }
        Some(t_overlap)
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn test_part1() {
            let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"; // test case input

            let answer = solve(input); // Get answer

            let answer = match answer {
                Some(value) => value,
                None => 0,
            };

            assert_eq!(answer, 2); // test your answer agains test case answer
        }
    }
}

pub mod part2 {
    pub fn solve(input: &str) -> Option<usize> {
                let lines = input.split("\n");



        let mut t_overlap = 0;
        for i in lines {
            let pair = i.split(",");

            let e1;//pair.next().unwrap();
            let e2;
            let mut temp = Vec::new();
            for i in pair {
                temp.push(i);
            }
            e1 = temp[0];
            e2 = temp[1];
            let range1 = e1.split("-");
            let mut range_a1 = Vec::new();
            for j in range1 {
                range_a1.push(j);
            }

            let mut el1 = 0;
            let mut eh1 = 0;
            if range_a1.len() == 2 {
                el1 = range_a1[0].parse::<usize>().unwrap();//range1.next().unwrap();
                eh1 = range_a1[1].parse::<usize>().unwrap();//range1.next().unwrap();
            }

            //let e2 = pair.next().unwrap();
            let range2 = e2.split("-");
            let mut range_a2 = Vec::new();
            for j in range2 {
                range_a2.push(j);
            }

            let mut el2 = 0;
            let mut eh2 = 0;
            if range_a2.len() == 2 {

                el2 = range_a2[0].parse::<usize>().unwrap();//range1.next().unwrap();
                eh2 = range_a2[1].parse::<usize>().unwrap();//range1.next().unwrap();
            }
            //let el2 = range2.next().unwrap();
            //let eh2 = range2.next().unwrap();

                //println!("range 1: {}, {}. range 2: {}, {}", el1, eh1, el2, eh2);
            if (eh1 >= el2 && eh1 <= eh2) || (eh2 >= el1 && eh2 <= eh1) {
                t_overlap += 1;
            }
        }
        Some(t_overlap)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn test_part2() {
            let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"; // test case input

            let answer = solve(input); // Get answer

            let answer = match answer {
                Some(value) => value,
                None => 0,
            };

            assert_eq!(answer, 4); // test your answer agains test case answer
        }
    }
}

fn main() {
    let input = match fs::read_to_string("input.txt") {
        Ok(input) => input,
        Err(_) => match fs::read_to_string("day04/input.txt") {
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

