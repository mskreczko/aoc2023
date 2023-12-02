use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::cmp::Ordering;

fn get_game_id(ln: &str) -> i32 {
    let pattern = Regex::new(r"\d+").unwrap();
    pattern.find(ln).map(|mat| mat.as_str().parse::<i32>().unwrap()).unwrap()
}

fn get_single_game_results(ln: &str) -> Vec<(&str, i32)> {
    let games: Vec<&str> = ln.split(",").collect();

    let number_pattern = Regex::new(r"\d+ ").unwrap();
    let color_pattern = Regex::new("red|green|blue").unwrap();

    let mut result: Vec<(&str, i32)> = Vec::new();
    for game in games {
        let number: i32 = number_pattern.find(game)
            .map(|mat| mat.as_str().trim()).unwrap().parse::<i32>().unwrap();
        let color: &str = color_pattern.find(game).map(|mat| mat.as_str()).unwrap();
        result.push((color, number));
    }

    result

}

fn solve_first() -> i32 {
    let file = File::open("/tmp/input").unwrap();
    let reader = BufReader::new(file);

    let mut result: i32 = 0;
    for line in reader.lines() {
        let ln = line.expect("Failed to read line");
        let games: Vec<&str> = ln.split(";").collect();
        let game_id: i32 = get_game_id(&ln);
        
        let mut possible: bool = true;
        for game in games {
            let game_results: Vec<(&str, i32)> = get_single_game_results(game); 
            for res in game_results {
                match res.0 {
                    "red" => match res.1.cmp(&12) {
                        Ordering::Greater => possible = false,
                        Ordering::Less | Ordering::Equal => ()},
                    "green" => match res.1.cmp(&13) {
                        Ordering::Greater => possible = false,
                        Ordering::Less | Ordering::Equal => ()},
                    "blue" => match res.1.cmp(&14) {
                        Ordering::Greater => possible = false,
                        Ordering::Less | Ordering::Equal => ()},
                    &_ => todo!()
                }
            }
        }
        if possible {
            result += game_id;
        }
    }
    result
}

fn solve_second() -> i32 {
    let file = File::open("/tmp/input").unwrap();
    let reader = BufReader::new(file);

    let mut result: i32 = 0;
    for line in reader.lines() {
        let ln = line.expect("Failed to read line");
        let games: Vec<&str> = ln.split(";").collect();
        
        let mut r_max: i32 = -1;
        let mut g_max: i32 = -1;
        let mut b_max: i32 = -1;
        for game in games {
            let game_results: Vec<(&str, i32)> = get_single_game_results(game); 
            for res in game_results {
                match res.0 {
                    "red" => match res.1.cmp(&r_max) {
                        Ordering::Greater => r_max = res.1,
                        Ordering::Less | Ordering::Equal => ()},
                    "green" => match res.1.cmp(&g_max) {
                        Ordering::Greater => g_max = res.1,
                        Ordering::Less | Ordering::Equal => ()},
                    "blue" => match res.1.cmp(&b_max) {
                        Ordering::Greater => b_max = res.1,
                        Ordering::Less | Ordering::Equal => ()},
                    &_ => todo!()
                }
            }
        }
        result += r_max * g_max * b_max;
    }

    result
}

fn main() {
    println!("Sol1: {}", solve_first());
    println!("Sol2: {}", solve_second());
}
