use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use regex::Regex;

fn parseLineWithDos(dont: &mut bool, line: &str) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    let reDo: Regex = Regex::new(r"do\(\)|(don't\(\))|mul\([0-9]+,[0-9]+\)").unwrap();
    let re : Regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let got: Vec<&str> = reDo.captures_iter(&line).map(|c| c.get(0).unwrap().as_str()).collect();
    let mut results: Vec<(usize, usize)> = vec!();
    let mut i = 0;
    for g in got.clone() {
        i = i + 1;
        if (g == "don't()" && *dont == false) {
            *dont = true
        } else if (g == "do()" && *dont == true) {
            *dont = false
        } else if (*dont == false && g.starts_with("mul")) {
            for (_, [left, right]) in re.captures_iter(g).map(|c| c.extract()) {
                results.push((left.parse().unwrap(), right.parse().unwrap()));
            }
        }
    }
    Ok(results)
}

fn readFile() -> Result<Vec<Vec<(usize, usize)>>, Box<dyn Error>> {
    let file = File::open("src/assets/day3.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());
    let mut results: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut dont = false;
    for line in lines {
        results.push(parseLineWithDos(&mut dont, &line)?);
    }
    Ok(results)
}

fn sumAllLines(results: Vec<Vec<(usize, usize)>>) -> Result<usize, Box<dyn Error>> {
    Ok(results.iter().map(|line|
        line.iter().map(|tupple| tupple.0 * tupple.1).collect::<Vec<usize>>().iter().sum()
    ).collect::<Vec<usize>>().iter().sum())
}

fn main() {
    let lines = readFile().unwrap();
    let total = sumAllLines(lines).unwrap();
    println!("total {:}", total)
}
