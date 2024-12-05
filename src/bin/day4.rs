use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use regex::Regex;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
enum Direction {
    ZERO,
    Pi4,
    Pi2,
    ThreePi4,
    Pi,
    FivePi4,
    ThreePi2,
    SevenPi4
}

fn x_mult(dir: &Direction) -> i16 {
    match dir {
        Direction::ZERO => 1, Direction::Pi4 => 1, Direction::Pi2 => 0, Direction::ThreePi4 => -1,
        Direction::Pi => -1, Direction::FivePi4 => -1, Direction::ThreePi2 => 0, Direction::SevenPi4 => 1
    }
}

fn y_mult(dir: &Direction) -> i16 {
    match dir {
        Direction::ZERO => 0, Direction::Pi4 => 1, Direction::Pi2 => 1, Direction::ThreePi4 => 1,
        Direction::Pi => 0, Direction::FivePi4 => -1, Direction::ThreePi2 => -1, Direction::SevenPi4 => -1
    }
}

#[derive(Debug)]
struct Cell { x: i16, y: i16 }

impl Cell {
    fn next_cell(&self, dir: &Direction) -> Cell {
        let next_x = self.x + 1 * x_mult(dir);
        let next_y = self.y + 1 * y_mult(dir);
        Cell { x: next_x, y: next_y }
    }
}
struct Matrix { all_chars: Vec<Vec<char>> }

impl Matrix {

    fn char_for(&self, cell: &Cell) -> char {
        self.all_chars[cell.x as usize][cell.y as usize]
    }

    fn valid(&self, cell: &Cell) -> bool {
        if (cell.x < 0 || cell.x as usize > self.all_chars.len() - 1) { false }
        else if (cell.y < 0 || cell.y as usize > self.all_chars[0].len() - 1) { false }
        else { true }
    }

    fn valid_cells(&self, cells: Vec<&Cell>) -> bool {
        cells.iter().all(|c| self.valid(c))
    }

    fn matches_xmas(&self, x_cell: &Cell) -> usize {
        Direction::iter().filter(|dir| self.matches_xmas_cells(dir, x_cell)).count()
    }

    fn matches_xmas_cells(&self, dir: &Direction, x_cell: &Cell) -> bool {
        let m_cell = &x_cell.next_cell(dir);
        let a_cell = &m_cell.next_cell(dir);
        let s_cell = &a_cell.next_cell(dir);
        if (! self.valid_cells(vec![x_cell, m_cell, a_cell, s_cell])) {
            return false
        }
        (self.char_for(x_cell) == 'X' && self.char_for(m_cell) == 'M' && self.char_for(a_cell) == 'A' && self.char_for(s_cell) == 'S')
    }

    fn matches_cross(&self, a_cell: &Cell) -> usize {
        if (self.char_for(a_cell) != 'A') {
            return 0
        }
        let tl_cell = &a_cell.next_cell(&Direction::ThreePi4);
        let tr_cell = &a_cell.next_cell(&Direction::Pi4);
        let bl_cell = &a_cell.next_cell(&Direction::FivePi4);
        let br_cell = &a_cell.next_cell(&Direction::SevenPi4);
        if ! self.valid_cells(vec![a_cell, tl_cell, tr_cell, bl_cell, br_cell]) {
            return 0
        }
        let top_match =
            (self.char_for(tl_cell) == 'M' && self.char_for(br_cell) == 'S') ||
                (self.char_for(tl_cell) == 'S' && self.char_for(br_cell) == 'M');
        let bottom_match =
            (self.char_for(bl_cell) == 'M' && self.char_for(tr_cell) == 'S') ||
                (self.char_for(bl_cell) == 'S' && self.char_for(tr_cell) == 'M');
        if (bottom_match && top_match) { 1 } else { 0 }
    }
}

fn transpose<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    // Handle empty matrix case
    if matrix.is_empty() {
        return Vec::new();
    }

    // Determine the dimensions of the transposed matrix
    let rows = matrix.len();
    let cols = matrix[0].len();
    println!("transposing row={:} col={:}", rows, cols);
    // Create a new matrix with swapped dimensions
    (0..cols)
        .map(|col| {
            (0..rows)
                .map(|row| matrix[row][col].clone())
                .collect()
        })
        .collect()
}

fn read_file() -> Vec<Vec<char>> {
    let file = File::open("src/assets/day4.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());
    let mut results: Vec<Vec<char>> = Vec::new();
    for line in lines {
        results.push(line.chars().collect());
    }
    transpose(&results)
}

fn loop_through<F>(m: &Matrix, match_count: F) -> usize
    where F: Fn(&Matrix, &Cell) -> usize
{
    let mut counter = 0;
    for x in (0..m.all_chars.len()) {
        for y in (0..m.all_chars[0].len()) {
            let cell = Cell { x: x as i16, y: y as i16};
            counter = counter + match_count(m, &cell);
        }
    }
    counter
}

fn main() {
    match env::current_dir() {
        Ok(path) => println!("Current path: {}", path.display()),
        Err(e) => println!("Error getting current directory: {}", e),
    }

    let cells = read_file();
    let matrix = Matrix { all_chars: cells };
    let total_xmas = loop_through(&matrix, Matrix::matches_xmas);
    let total_cross = loop_through(&matrix, Matrix::matches_cross);
    println!("total_xmas {:}", total_xmas);
    println!("total_xmas {:}", total_cross);
}
