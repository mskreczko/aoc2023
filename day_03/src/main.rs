use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::collections::HashMap;

fn find_neighbors(grid: &Vec<String>, start_y: usize, start_x: usize, length: usize) -> bool {
    for y in start_y.saturating_sub(1)..=start_y + 1 {
        for x in start_x.saturating_sub(1)..=length {
            if y < grid.len() && x < grid[y].len() {
                if !grid[y].chars().nth(x).unwrap().is_numeric() && grid[y].chars().nth(x).unwrap() != '.' {
                    return true;
                }
            }
        }
    }
    false
}

fn find_gears(grid: &Vec<String>, gears: &mut HashMap<(usize, usize), Vec<i32>>,
              start_y: usize, start_x: usize, length: usize, num: i32) {
    for y in start_y.saturating_sub(1)..=start_y + 1 {
        for x in start_x.saturating_sub(1)..=length {
            if y < grid.len() && x < grid[y].len() {
                if !grid[y].chars().nth(x).unwrap().is_numeric() && grid[y].chars().nth(x).unwrap() != '.' {
                    if grid[y].chars().nth(x).unwrap() == '*' {
                        gears.entry((y, x)).or_insert(vec![]).push(num);
                    }
                }
            }
        }
    }
}

fn solve_first() -> i32 {
    let file = File::open("/tmp/input").unwrap();
    let reader = BufReader::new(file);
    let pattern = Regex::new(r"\d+").unwrap();
    
    let mut result: i32 = 0;

    let lines: Vec<String> = reader.lines().map(|ln| ln.expect("Failed to read line")).collect();

    for (index, line) in lines.clone().into_iter().enumerate() {
        for num in pattern.find_iter(&line) {
            if find_neighbors(&lines, index, num.start(), num.end()) {
                result += line[num.start()..num.end()].parse::<i32>().unwrap();
            }
        }
    }

    result
}

fn solve_second() -> i32 {
    let file = File::open("/tmp/input").unwrap();
    let reader = BufReader::new(file);
    let pattern = Regex::new(r"\d+").unwrap();

    let mut result: i32 = 0;

    let lines: Vec<String> = reader.lines().map(|ln| ln.expect("Failed to read line")).collect();
    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for (index, line) in lines.clone().into_iter().enumerate() {
        for num in pattern.find_iter(&line) {
            let number: i32 = line[num.start()..num.end()].parse::<i32>().unwrap();
            find_gears(&lines, &mut gears, index, num.start(), num.end(), number)
        }
    }

    for (_k, v) in gears {
        if v.len() == 2 {
            result += v[0] * v[1];
        }
    }

    result
}

fn main() {
    println!("Sol1: {}", solve_first());
    println!("Sol2: {}", solve_second());
}
