use core::str::FromStr;
use std::io::{BufRead, BufReader, Read};

use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;

const TOTAL_CUBES: Cubes = Cubes {
    red: 12,
    green: 13,
    blue: 14,
};

fn main() -> anyhow::Result<()> {
    let input = std::fs::File::open("input.txt").unwrap();
    let input = BufReader::new(input);
    let input = input.lines().flatten();
    let games: Vec<Game> = input.flat_map(|l| l.parse()).collect();
    let valid_games = games
        .iter()
        .filter(|g| {
            !g.draws.iter().any(|d| {
                d.cube.red > TOTAL_CUBES.red
                    || d.cube.green > TOTAL_CUBES.green
                    || d.cube.blue > TOTAL_CUBES.blue
            })
        })
        .collect::<Vec<_>>();
    let total = valid_games.iter().map(|g| g.id).sum::<u32>();
    println!("Solution 1: {}", total);

    let least_cube_power: u32 = games
        .iter()
        .map(|game| {
            let cube = game.draws.iter().fold(
                Cubes {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |acc, draw| {
                    let red = acc.red.max(draw.cube.red);
                    let green = acc.green.max(draw.cube.green);
                    let blue = acc.blue.max(draw.cube.blue);
                    Cubes { red, green, blue }
                },
            );
            cube.red as u32 * cube.green as u32 * cube.blue as u32
        })
        .sum();
    println!("Solution 2: {}", least_cube_power);

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub struct Cubes {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub draws: Vec<Draw>,
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let res = game(input);
        match res {
            Ok((_, game)) => Ok(game),
            Err(e) => Err(anyhow::anyhow!("Error parsing: {:?}", e)),
        }
    }
}

#[derive(Debug)]
pub struct Draw {
    pub cube: Cubes,
}

pub fn game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game")(input)?;
    let (input, _) = space1(input)?;
    let (input, id) = digit1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, draws) = separated_list1(tag(";"), draw)(input)?;
    Ok((
        input,
        Game {
            id: id.parse().expect("id is a number"),
            draws,
        },
    ))
}

pub fn draw(input: &str) -> IResult<&str, Draw> {
    let mut cubes = Cubes {
        red: 0,
        green: 0,
        blue: 0,
    };
    let (input, mut c) = separated_list1(tag(","), cube)(input)?;
    while let Some(i) = c.pop() {
        match i.0.as_str() {
            "red" => cubes.red += i.1,
            "green" => cubes.green += i.1,
            "blue" => cubes.blue += i.1,
            _ => unreachable!(),
        }
    }
    Ok((input, Draw { cube: cubes }))
}

pub fn cube(input: &str) -> IResult<&str, (String, u8)> {
    let (input, _) = space0(input)?;
    let (input, count) = map_res(digit1, str::parse)(input)?;
    let (input, _) = space0(input)?;
    let (input, color) = alpha1(input)?;
    let (input, _) = space0(input)?;
    Ok((input, (color.to_string(), count)))
}
