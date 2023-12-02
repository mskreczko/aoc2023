use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn solve_first() -> i32 {
    let file = File::open("/tmp/input").unwrap();
    let reader = BufReader::new(file);
    
    let mut calibration_values: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let first = line.as_ref().unwrap().chars().find(|c| c.is_digit(10));
        let last = line.as_ref().unwrap().chars().rev().find(|c| c.is_digit(10));
        calibration_values.push(format!("{}{}", first.unwrap(), last.unwrap()).parse::<i32>().unwrap());
    }

    calibration_values.iter().sum()
}

fn find_occurences(line: &str, reverse: bool) -> Option<String> {
    let digits = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let rp = r"\d|";
    if reverse {
        let digits = vec!["eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"];
        let line_reversed: String = line.clone().chars().rev().collect();
        let pattern = Regex::new(&format!("{}{}", rp, &digits.join("|"))).unwrap();
        let result = pattern.find(&line_reversed).map(|mat| mat.as_str());
        return Some(result.unwrap().to_string());
    }
    let pattern = Regex::new(&format!("{}{}", rp, &digits.join("|"))).unwrap();
    Some(pattern.find(line).map(|mat| mat.as_str()).unwrap().to_string())
}

fn match_digit(digit: &str) -> &str {
    match digit {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        "eno" => "1",
        "owt" => "2",
        "eerht" => "3",
        "ruof" => "4",
        "evif" => "5",
        "xis" => "6",
        "neves" => "7",
        "thgie" => "8",
        "enin" => "9",
        &_ => digit
    }
}

fn solve_second() -> i32 {
    let file = File::open("/tmp/input").unwrap();
    let reader = BufReader::new(file);


    let mut calibration_values: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let ln = line.expect("Failed to read line");
        let first = find_occurences(&ln, false);
        let last = find_occurences(&ln, true);

        let mut number: String = String::from(match_digit(&first.unwrap()));
        number.push_str(match_digit(&last.unwrap()));
        calibration_values.push(number.parse::<i32>().unwrap());

    }
    calibration_values.iter().sum()
}

fn main() {
    println!("Sol1: {}", solve_first());
    println!("Sol2: {}", solve_second());
}
