use std::{
    io::{stdin, BufRead, BufReader},
    num::ParseIntError,
    str::FromStr,
};

pub struct History(Vec<i64>);

impl FromStr for History {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(History(
            input
                .split_whitespace()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl History {
    fn next_value(&self) -> i64 {
        if self.0.iter().all(|i| *i == 0) {
            0
        } else {
            self.0.last().unwrap() + self.derivative().next_value()
        }
    }

    fn prev_value(&self) -> i64 {
        if self.0.iter().all(|i| *i == 0) {
            0
        } else {
            self.0.first().unwrap() - self.derivative().prev_value()
        }
    }

    fn derivative(&self) -> Self {
        Self(
            self.0
                .iter()
                .zip(self.0.iter().skip(1))
                .map(|(prev, next)| next - prev)
                .collect(),
        )
    }
}

#[test]
fn can_parse() {
    for line in include_str!("../sample.txt").lines() {
        let _history: History = line.parse().unwrap();
    }
}

#[test]
fn can_solve() {
    let histories: Vec<History> = include_str!("../sample.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    assert_eq!(histories[0].next_value(), 18);
    assert_eq!(histories[1].next_value(), 28);
    assert_eq!(histories[2].next_value(), 68);
}

#[test]
fn can_solve2() {
    let histories: Vec<History> = include_str!("../sample.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    assert_eq!(histories[2].prev_value(), 5);
}

fn main() {
    let input = BufReader::new(stdin());
    let histories: Vec<History> = input
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let sum_next: i64 = histories.iter().map(|h| h.next_value()).sum();
    println!("sum_next {sum_next}");

    let sum_prev: i64 = histories.iter().map(|h| h.prev_value()).sum();
    println!("sum_prev {sum_prev}");
}
