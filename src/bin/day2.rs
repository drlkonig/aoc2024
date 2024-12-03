use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Mul;
use std::path::Path;
use itertools::Itertools;

fn orientation(numbers: &Vec<i32>) -> Result<bool, Box<dyn Error>> {
    let mut iter = numbers.iter();
    let first = *iter.next().unwrap();
    let second = *iter.next().unwrap();
    Ok(second > first)
}

fn isSafeCheck(numbers: &Vec<i32>) -> Result<bool, Box<dyn Error>> {
    let orientation = orientation(numbers)?;
    let mut safe = true;
    if (numbers.len() < 3) {
        return Ok(true);
    }

    let mut i = 0;
    for j in 0..numbers.len() - 1 {
        if (orientation) {
            safe = numbers[j + 1] > numbers[j] && numbers[j + 1] - numbers[j] < 4
        } else {
            safe = numbers[j + 1] < numbers[j] && numbers[j] - numbers[j + 1] < 4
        }
        if (!safe) {
            break;
        } else {
            i = j
        }
    }
    Ok(safe)
}

fn isSafe(problemDampening: bool, numbers: &Vec<i32>) -> Result<bool, Box<dyn Error>> {
    let result = isSafeCheck(numbers)?;
    if (problemDampening && ! result) {
        for index in 0..numbers.len() {
            let mut clone = numbers.clone();
            clone.remove(index);
            if (isSafeCheck(&clone).unwrap()) {
                return Ok(true)
            }
        }
    }
    Ok(result)
}

fn readFile() -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let file = File::open("src/assets/day2.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());
    let mut results: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let mut words = line.split_whitespace();
        let numbers = words.into_iter().map(|x| x.parse().unwrap()).collect();
        results.push(numbers);
    }
    Ok(results)
}

fn main() {
    let rows = readFile().unwrap();
    let safeRowCount = rows.iter().enumerate().filter(|tuple| isSafe(true, tuple.1).unwrap()).count();
    println!("Length found total={:}, safe={:}", rows.len(), safeRowCount)
}
