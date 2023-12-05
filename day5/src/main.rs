use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::multi::*;
use nom::*;

#[derive(Debug)]
pub struct MapItem {
    source: u64,
    target: u64,
    range: u64,
}

#[derive(Debug)]
pub struct Map {
    source: String,
    target: String,
    items: Vec<MapItem>,
}

#[derive(Debug)]
pub struct Seeds {
    seeds: Vec<u64>,
}

#[derive(Debug)]
pub struct Garden {
    seeds: Seeds,
    maps: Vec<Map>,
}

impl Map {
    pub fn map(&self, source: u64) -> u64 {
        self.items
            .iter()
            .find(|item| item.source <= source && item.source + item.range >= source)
            .map(|item| item.target + source - item.source)
            .unwrap_or(source)
    }
    pub fn print_table(&self, range: core::ops::Range<u64>) {
        println!("{} to {}", self.source, self.target);
        for i in range {
            println!("{} -> {}", i, self.map(i));
        }
    }
}

impl Garden {
    pub fn map(&self, source: &str, target: &str) -> &Map {
        self.maps
            .iter()
            .find(|map| map.source == *source && map.target == *target)
            .expect("invalid souce or target")
    }
}

fn main() {
    let garden = parse_garden(include_str!("../input.txt")).unwrap().1;
    let locations: Vec<_> = garden
        .seeds
        .seeds
        .iter()
        .cloned()
        .map(|seed| garden.map("seed", "soil").map(seed))
        // .inspect(|soil| println!("soil: {soil}"))
        .map(|soil| garden.map("soil", "fertilizer").map(soil))
        .map(|fert| garden.map("fertilizer", "water").map(fert))
        .map(|water| garden.map("water", "light").map(water))
        .map(|light| garden.map("light", "temperature").map(light))
        .map(|temp| garden.map("temperature", "humidity").map(temp))
        .map(|hum| garden.map("humidity", "location").map(hum))
        .collect();
    let small_location = locations.iter().fold(u64::MAX, |acc, &x| acc.min(x));
    use rayon::prelude::*;
    println!("Solution 1: {}", small_location);
    let locations: Vec<_> = garden
        .seeds
        .seeds
        .chunks_exact(2)
        .flat_map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|seed| garden.map("seed", "soil").map(seed))
        .map(|soil| garden.map("soil", "fertilizer").map(soil))
        .map(|fert| garden.map("fertilizer", "water").map(fert))
        .map(|water| garden.map("water", "light").map(water))
        .map(|light| garden.map("light", "temperature").map(light))
        .map(|temp| garden.map("temperature", "humidity").map(temp))
        .map(|hum| garden.map("humidity", "location").map(hum))
        .collect();
    let small_location = locations.iter().fold(u64::MAX, |acc, &x| acc.min(x));
    println!("Solution 2: {}", small_location);
}

pub fn parse_garden(input: &str) -> IResult<&str, Garden> {
    let (input, seeds) = parse_seeds(input)?;
    let (input, _) = multispace1(input)?;
    let (input, maps) = separated_list1(multispace1, parse_map)(input)?;
    Ok((input, Garden { seeds, maps }))
}

pub fn parse_seeds(input: &str) -> IResult<&str, Seeds> {
    let (input, _) = tag("seeds:")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, seeds) = separated_list1(multispace1, u64)(input)?;
    Ok((input, Seeds { seeds }))
}

pub fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, source) = take_until("-to-")(input)?;
    let (input, _) = tag("-to-")(input)?;
    let (input, target) = take_until(" ")(input)?;
    let (input, _) = tag(" map:")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) = separated_list1(tag("\n"), parse_map_item)(input)?;
    Ok((
        input,
        Map {
            source: source.to_string(),
            target: target.to_string(),
            items,
        },
    ))
}

pub fn parse_map_item(input: &str) -> IResult<&str, MapItem> {
    let (input, target) = u64(input)?;
    let (input, _) = multispace1(input)?;
    let (input, source) = u64(input)?;
    let (input, _) = multispace1(input)?;
    let (input, range) = u64(input)?;
    // let (input, _) = multispace1(input)?;
    Ok((
        input,
        MapItem {
            source,
            target,
            range,
        },
    ))
}
