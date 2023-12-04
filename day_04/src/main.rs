use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

fn solve_first() -> i32 {
    let file = File::open("/tmp/input").unwrap();
    let reader = BufReader::new(file);
    let pattern = Regex::new(r"\d+").unwrap();

    let mut result: i32 = 0;

    let lines: Vec<String> = reader.lines().map(|ln| ln.expect("Failed to read line")).collect();

    for line in lines {
        let parts = line.split(" | ").collect::<Vec<&str>>();
        let winning_part = parts[0].split(": ").collect::<Vec<&str>>();
        let my_part = parts[1];

        let winning_numbers = pattern.find_iter(winning_part[1])
            .map(|mat| mat.as_str()).collect::<HashSet<&str>>();
        let my_numbers = pattern.find_iter(my_part)
            .map(|mat| mat.as_str()).collect::<HashSet<&str>>();

        let mut tmp: i32 = 0;
        for my_num in my_numbers {
            if winning_numbers.contains(&my_num) {
                if tmp == 0 {
                    tmp = 1;
                } else {
                    tmp *= 2;
                }
            }
        }
        result += tmp;
    }

    result
}

fn get_matches(x: &HashSet<String>, y: &HashSet<String>) -> usize {
    let mut res: usize = 0;
    for num in y {
        if x.contains(num) {
            res += 1;
        }
    }
    res
}

fn solve_second() -> i32 {
    let file = File::open("/tmp/input").unwrap();
    let reader = BufReader::new(file);
    let pattern = Regex::new(r"\d+").unwrap();

    let lines: Vec<String> = reader.lines().map(|ln| ln.expect("Failed to read line")).collect();

    let mut winning: Vec<HashSet<String>> = Vec::new();
    let mut mine: Vec<HashSet<String>> = Vec::new();
    for line in lines {
        let parts = line.split(" | ").collect::<Vec<&str>>();
        let winning_part = parts[0].split(": ").collect::<Vec<&str>>();
        let my_part = parts[1];

        let winning_numbers = pattern.find_iter(winning_part[1])
            .map(|mat| mat.as_str().to_string()).collect::<HashSet<String>>();
        let my_numbers = pattern.find_iter(my_part)
            .map(|mat| mat.as_str().to_string()).collect::<HashSet<String>>();

        winning.push(winning_numbers);
        mine.push(my_numbers);
    }

    let mut cards_instances = vec![1; winning.len()];
    for i in 0..winning.len() {
        for j in (i + 1)..=(i + get_matches(&winning[i], &mine[i])) {
            cards_instances[j] += cards_instances[i];
        }
    }

    cards_instances.iter().sum()
}

fn main() {
    println!("Sol1: {}", solve_first());
    println!("Sol2: {}", solve_second());
}
