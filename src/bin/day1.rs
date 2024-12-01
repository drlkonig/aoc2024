use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Mul;
use std::path::Path;
use itertools::Itertools;

fn calculate_distance(mut left: Vec<i32>, mut right: Vec<i32>) -> Result<i32, Box<dyn Error>> {
    left.sort();
    right.sort();
    let result: Vec<i32> = left.iter().enumerate().map(|(index, &val)| {
        (right[index] - val).abs()
    }).collect();
    Ok(result.iter().sum())
}

fn calculate_similarity(mut left: Vec<i32>, mut right: Vec<i32>) -> Result<i32, Box<dyn Error>> {
    let map = right.into_iter().into_group_map_by(|element| *element);
    let mut result = 0;
    for i in left {
        let lookup = map.get(&i);
        if (lookup.is_some()) {
            result = result + (i.mul(lookup.unwrap().len() as i32));
            println!("found: {:} times {:} result {:}", i, lookup.unwrap().len(), result)
        }
    }
    Ok(result)
}

fn readFile() -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let file = File::open("src/assets/day1.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut i = 0;
    for line in lines {
        i = i + 1;
        let mut words = line.split_whitespace();
        left.push(words.next().unwrap().parse().unwrap());
        right.push(words.next().unwrap().parse().unwrap());
    }
    Ok((left, right))
}

fn problem1() {
    match env::current_dir() {
        Ok(path) => println!("Current path: {}", path.display()),
        Err(e) => println!("Error getting current directory: {}", e),
    }

    let tuple = readFile().unwrap();
    let distance = calculate_distance(tuple.0, tuple.1);

    println!("Distance {:?}", distance);
}

fn main() {
    match env::current_dir() {
        Ok(path) => println!("Current path: {}", path.display()),
        Err(e) => println!("Error getting current directory: {}", e),
    }

    let tuple = readFile().unwrap();
    let distance = calculate_similarity(tuple.0, tuple.1);

    println!("Distance {:?}", distance);
}
