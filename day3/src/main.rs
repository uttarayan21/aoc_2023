use std::collections::{HashMap, HashSet};

use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::Offset;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read("input.txt")?;
    let engine = Engine::from_input(&input);
    let numbers = engine.numbers().expect("Numbers");
    let total = numbers
        .iter()
        .filter(|number| engine.is_part_number(number))
        .map(|number| number.value)
        .sum::<u64>();
    println!("Solution 1: {}", total);
    let mut sets = HashMap::new();
    numbers.iter().for_each(|number| {
        let symbols = engine.number_symbols(number);
        for symbol in symbols {
            sets.entry(symbol)
                .or_insert_with(HashSet::new)
                .insert(number);
        }
    });
    let gears = sets.iter().filter_map(|(symbol, nums)| {
        if nums.len() == 2 && symbol.symbol == b'*' {
            Some(nums.iter().map(|n| n.value).product::<u64>())
        } else {
            None
        }
    });
    let total = gears.sum::<u64>();
    println!("Solution 2: {}", total);

    Ok(())
}

#[derive(Debug)]
pub struct Engine {
    pub row: usize,
    pub col: usize,
    pub grid: Vec<u8>,
}

impl Engine {
    pub fn from_input(input: &[u8]) -> Self {
        let grid = input.to_vec();
        let mut row = 0;
        let mut col = 0;
        input.split(|&c| c == b'\n').for_each(|line| {
            col = col.max(line.len());
            if !line.is_empty() {
                row += 1;
            }
        });
        let col = col + 1;
        // dbg!(row, col);
        Self { row, col, grid }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Number {
    offset: usize,
    size: usize,
    value: u64,
}

pub fn parse(input: &[u8]) -> nom::IResult<&[u8], Number> {
    let original = input;
    let ignore = take_till(|c: u8| c.is_ascii_digit());
    let mut part = nom::sequence::preceded(ignore, u64);
    let (input, value) = part(input)?;
    let offset = original.offset(input);
    let size = value.to_string().len();
    Ok((
        input,
        Number {
            offset,
            size,
            value,
        },
    ))
}

pub fn offset_to_row_col(offset: usize, col: usize) -> (usize, usize) {
    let row = offset / col;
    let col = offset % col;
    (row, col)
}

pub fn row_col_to_offset(row: usize, col: usize, col_size: usize) -> usize {
    row * col_size + col
}

impl Engine {
    pub fn numbers(&self) -> Option<Vec<Number>> {
        let input = &self.grid[..];
        let (_, numbers) = nom::multi::fold_many1(parse, Vec::<Number>::new, |mut acc, item| {
            let last = acc.last().map(|n| n.offset + n.size).unwrap_or(0);
            let offset = last + item.offset - item.size;
            acc.push(Number {
                offset,
                size: item.size,
                value: item.value,
            });
            acc
        })(input)
        .ok()?;
        Some(numbers)
    }

    pub fn is_symbol_adjacent(&self, row: usize, col: usize) -> bool {
        for r in row.saturating_sub(1)..=(row + 1).min(self.row - 1) {
            for c in col.saturating_sub(1)..=(col + 1).min(self.col - 1) {
                if row == r && col == c {
                    continue;
                }
                let offset = row_col_to_offset(r, c, self.col);
                let cell = self.grid[offset];

                // println!("{}x{}: {:?}", r, c, cell as char);
                if cell.is_ascii_punctuation() && cell != b'.' {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_part_number(&self, number: &Number) -> bool {
        for size in 0..number.size {
            let offset = number.offset + size;
            // dbg!(self.col);
            // println!(
            //     "Char: {:?} of {} at offset {}",
            //     self.grid[offset] as char, number.value, offset
            // );
            let (row, col) = offset_to_row_col(offset, self.col);
            // println!("{row}x{col}: {}", number.value);
            if self.is_symbol_adjacent(row, col) {
                return true;
            }
        }
        false
    }

    pub fn symbols(&self, offset: usize) -> HashSet<Symbol> {
        let (row, col) = offset_to_row_col(offset, self.col);
        let mut results = HashSet::new();

        for r in row.saturating_sub(1)..=(row + 1).min(self.row - 1) {
            for c in col.saturating_sub(1)..=(col + 1).min(self.col - 1) {
                if row == r && col == c {
                    continue;
                }
                let offset = row_col_to_offset(r, c, self.col);
                let cell = self.grid[offset];

                // println!("{}x{}: {:?}", r, c, cell as char);
                if cell.is_ascii_punctuation() && cell != b'.' {
                    results.insert(Symbol {
                        offset,
                        symbol: cell,
                    });
                }
            }
        }

        results
    }

    pub fn number_symbols(&self, number: &Number) -> HashSet<Symbol> {
        let mut results = HashSet::new();
        for size in 0..number.size {
            let offset = number.offset + size;
            let symbols = self.symbols(offset);
            results.extend(symbols);
        }
        results
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Symbol {
    offset: usize,
    symbol: u8,
}
