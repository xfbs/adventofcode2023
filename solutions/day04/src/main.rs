use std::{
    collections::BTreeSet,
    io::{stdin, BufRead, BufReader},
    num::ParseIntError,
    str::FromStr,
};
use thiserror::Error;

fn main() {
    let input = BufReader::new(stdin());
    let points: u64 = input
        .lines()
        .map(|line| Card::from_str(&line.unwrap()).unwrap().points())
        .sum();
    println!("{points}");
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("missing colon separator")]
    MissingSeparator,
    #[error("missing card separator space")]
    MissingCardSpace,
    #[error("missing numbers separator")]
    MissingNumbersSeparator,
    #[error(transparent)]
    Parse(#[from] ParseIntError),
}

pub struct Card {
    pub number: u64,
    pub winning: Vec<u64>,
    pub selected: Vec<u64>,
}

impl Card {
    fn winning_selected(&self) -> usize {
        let winning: BTreeSet<_> = self.winning.iter().copied().collect();
        self.selected
            .iter()
            .filter(|num| winning.contains(num))
            .count()
    }

    fn points(&self) -> u64 {
        match self.winning_selected() {
            0 => 0,
            other => 1 << (other - 1),
        }
    }
}

impl FromStr for Card {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (card, numbers) = input.split_once(": ").ok_or(ParseError::MissingSeparator)?;
        let number = card
            .split_whitespace()
            .nth(1)
            .ok_or(ParseError::MissingCardSpace)?;
        let number = number.parse()?;
        let (winning, selected) = numbers
            .split_once(" | ")
            .ok_or(ParseError::MissingNumbersSeparator)?;
        let parse = |input: &str| {
            input
                .trim()
                .split_whitespace()
                .map(|number| number.parse())
                .collect::<Result<_, _>>()
        };
        Ok(Self {
            number,
            winning: parse(winning)?,
            selected: parse(selected)?,
        })
    }
}

#[test]
fn can_parse() {
    let input = include_str!("input.txt");
    for line in input.lines() {
        let _card: Card = line.parse().unwrap();
    }
}

#[test]
fn can_solve() {
    let cards: Vec<Card> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    assert_eq!(cards[0].winning_selected(), 4);
    assert_eq!(cards[0].points(), 8);
    assert_eq!(cards[1].winning_selected(), 2);
    assert_eq!(cards[1].points(), 2);
    assert_eq!(cards[2].winning_selected(), 2);
    assert_eq!(cards[2].points(), 2);
    assert_eq!(cards[3].winning_selected(), 1);
    assert_eq!(cards[3].points(), 1);
    assert_eq!(cards[4].winning_selected(), 0);
    assert_eq!(cards[4].points(), 0);
    assert_eq!(cards[5].winning_selected(), 0);
    assert_eq!(cards[5].points(), 0);
}
