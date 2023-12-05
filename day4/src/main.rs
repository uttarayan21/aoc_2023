use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let cards = input
        .split('\n')
        .flat_map(Card::from_str)
        .collect::<Vec<_>>();
    let points = cards
        .iter()
        .map(|card| card.winners())
        .filter(|c| *c > 0)
        .map(|count| 2u32.pow(count as u32 - 1))
        .sum::<u32>();
    println!("Solution: {}", points);
    let mut winners: Vec<(usize, usize)> =
        cards.into_iter().map(|card| (card.winners(), 1)).collect();
    for i in 0..winners.len() {
        let (w, count) = winners[i];
        winners
            .iter_mut()
            .skip(i + 1)
            .take(w)
            .for_each(|(_, c)| *c += count);
    }
    let points = winners.iter().map(|(_, count)| count).sum::<usize>();
    println!("Solution: {}", points);
}

#[derive(Debug)]
pub struct Card {
    id: u32,
    winners: HashSet<u32>,
    current: HashSet<u32>,
}

impl Card {
    pub fn winners(&self) -> usize {
        self.winners.intersection(&self.current).count()
    }

    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    // Takes a single line
    fn from_str(input: &str) -> Option<Self> {
        let (_, card) = parse(input).ok()?;
        Some(card)
    }
}

pub fn parse(input: &str) -> nom::IResult<&str, Card> {
    use nom::bytes::complete::*;
    use nom::character::complete::*;
    use nom::multi::*;
    use nom::sequence::*;

    let (input, _) = multispace0(input)?;
    let (input, _) = tag("Card")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, id) = u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, winners) = many1(delimited(multispace0, u32, multispace0))(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, current) = many1(delimited(multispace0, u32, multispace0))(input)?;
    Ok((
        input,
        Card {
            id,
            winners: winners.into_iter().collect(),
            current: current.into_iter().collect(),
        },
    ))
}
