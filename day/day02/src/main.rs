use std::{
    collections::BTreeMap,
    io::{stdin, BufRead, BufReader},
    num::ParseIntError,
    str::FromStr,
};
use strum::EnumString;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, EnumString)]
#[strum(serialize_all = "snake_case")]
enum Color {
    Blue,
    Red,
    Green,
}

struct Game {
    id: u64,
    reaches: Vec<Reach>,
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("missing game delimiter")]
    MissingGameDelimiter,
    #[error("missing game number")]
    MissingGameNumber,
    #[error(transparent)]
    ParseGameNumber(#[from] ParseIntError),
    #[error(transparent)]
    ParseReach(#[from] ParseReachError),
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use ParseError::*;

        let Some((game, reaches)) = input.split_once(": ") else {
            return Err(MissingGameDelimiter);
        };

        let id: u64 = game.split(" ").nth(1).ok_or(MissingGameNumber)?.parse()?;

        let reaches = reaches
            .trim()
            .split("; ")
            .map(Reach::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Game { id, reaches })
    }
}

#[derive(Error, Debug)]
enum ParseReachError {
    #[error("invalid reach")]
    Invalid,
    #[error("invalid number")]
    InvalidNumber(#[from] ParseIntError),
    #[error("invalid color")]
    InvalidColor(#[from] strum::ParseError),
}

struct Reach {
    colors: BTreeMap<Color, usize>,
}

impl FromStr for Reach {
    type Err = ParseReachError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use ParseReachError::*;
        let mut colors = BTreeMap::new();
        for color in input.split(", ") {
            let Some((number, color)) = color.split_once(" ") else {
                return Err(Invalid);
            };

            let number: usize = number.parse()?;
            let color: Color = color.parse()?;
            let entry = colors.entry(color).or_default();
            *entry += number;
        }
        Ok(Reach { colors })
    }
}

struct Config {
    colors: BTreeMap<Color, usize>,
}

impl Config {
    pub fn possible_reach(&self, reach: &Reach) -> bool {
        for (color, count) in reach.colors.iter() {
            let Some(possible) = self.colors.get(color) else {
                return false;
            };

            if count > possible {
                return false;
            }
        }

        true
    }

    pub fn possible_game(&self, game: &Game) -> bool {
        game.reaches.iter().all(|reach| self.possible_reach(reach))
    }
}

#[test]
fn can_parse() {
    let game: Game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        .parse()
        .unwrap();
    assert_eq!(game.id, 1);
    assert_eq!(game.reaches.len(), 3);
    assert_eq!(
        game.reaches[0].colors,
        [(Color::Blue, 3), (Color::Red, 4),].into()
    );
    assert_eq!(
        game.reaches[1].colors,
        [(Color::Red, 1), (Color::Green, 2), (Color::Blue, 6),].into()
    );
    assert_eq!(game.reaches[2].colors, [(Color::Green, 2),].into());
}

fn main() {
    let config = Config {
        colors: [(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)].into(),
    };

    let input = BufReader::new(stdin());

    let sum: u64 = input
        .lines()
        .map(|line| Game::from_str(&line.unwrap()).unwrap())
        .filter(|game| config.possible_game(game))
        .map(|game| game.id)
        .sum();

    println!("{sum}");
}
