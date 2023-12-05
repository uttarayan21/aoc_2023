use std::io::BufRead;

fn main() {
    let inputs = std::fs::File::open("input.txt").expect("Could not open file");
    let inputs = std::io::BufReader::new(inputs);
    let inputs = inputs.lines().map(|l| l.expect("Could not parse line"));
    let out = inputs
        .flat_map(|l| calibration(&l))
        .map(|num| num as u32)
        .sum::<u32>();
    println!("Solution 1: {}", out);

    let inputs = std::fs::File::open("input.txt").expect("Could not open file");
    let inputs = std::io::BufReader::new(inputs);
    let inputs = inputs.lines().map(|l| {
        let l = l.expect("Could not parse line");
        let l = l.replace("one", "o1ne");
        let l = l.replace("two", "t2wo");
        let l = l.replace("three", "th3ree");
        let l = l.replace("four", "fo4ur");
        let l = l.replace("five", "fi5ve");
        let l = l.replace("six", "s6ix");
        let l = l.replace("seven", "se7ven");
        let l = l.replace("eight", "ei8ght");
        l.replace("nine", "ni9ne")
    });

    let out = inputs
        .map(|l| calibration(&l).expect("wut"))
        .map(|num| num as u32)
        .sum::<u32>();
    println!("Solution 2: {}", out);
}

pub fn calibration(input: &str) -> Option<u8> {
    // get the first and the last number in the input string
    let first = input.chars().find(|c| c.is_ascii_digit())?;
    let last = input.chars().rev().find(|c| c.is_ascii_digit())?;
    // combine those two to get the number
    format!("{}{}", first, last).parse::<u8>().ok()
}
