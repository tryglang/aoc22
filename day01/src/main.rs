use std::{fs, process::exit};

fn main() {

    let input = get_input();

    match part1::solve(&input){
        Some(data) => println!("Part 1: {}", data),
        None => println!("Part 1: not done")
    }

    match part2::solve(&input) {
        Some(data) => println!("Part 2: {}", data),
        None => println!("Part 2: not done")
    }
}

pub mod part1 {
    pub fn solve(input: &Vec<String>) -> Option<usize> {

        let content = input;
    let mut bot_cals: Vec<usize> = Vec::new();
    let mut cal_amount = 0;
    for i in content {
        if i == "" {
            bot_cals.push(cal_amount);
            cal_amount = 0;
        } else {
            let cals = i.parse::<usize>().unwrap();
            cal_amount += cals;
        }
    }

    bot_cals.sort_by(|a, b| b.cmp(a));
    let result = bot_cals[0];
        Some(result)
    }
}

pub mod part2 {
    pub fn solve(input: &Vec<String>) -> Option<usize> {
        let content = input;
    let mut bot_cals: Vec<usize> = Vec::new();
    let mut cal_amount = 0;
    for i in content {
        if i == "" {
            bot_cals.push(cal_amount);
            cal_amount = 0;
        } else {
            let cals = i.parse::<usize>().unwrap();
            cal_amount += cals;
        }
    }

    bot_cals.sort_by(|a, b| b.cmp(a));
    let mut sum = 0;
    for i in 0..3 {
        sum += bot_cals[i];

    }

        Some(sum)
    }
}


fn get_input() -> Vec<String>{
    let file = "input.txt";

    let content = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(_) => match fs::read_to_string("day01/".to_owned() + file) {
            Ok(content) => content,
            Err(_) => {
                println!("Could not find input file");
                exit(1);
            }
        }
    };
    
    let lines =  content.split("\n");

    let mut result = Vec::new();
    
    for i in lines{
        result.push(String::from(i));
    }

    result

}
