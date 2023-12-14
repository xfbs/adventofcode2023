use std::{collections::BTreeMap, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

pub struct Directions(Vec<Direction>);

impl FromStr for Directions {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input
            .chars()
            .map(|c| match c {
                'L' => Ok(Direction::Left),
                'R' => Ok(Direction::Right),
                _ => Err(()),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(Self)
    }
}

#[derive(Default, Debug, Clone)]
pub struct Nodes(BTreeMap<String, [String; 2]>);

impl FromStr for Nodes {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut nodes = Nodes::default();
        for line in input.lines() {
            let (name, options) = line.split_once(" = ").unwrap();
            let options = options.trim_start_matches("(").trim_end_matches(")");
            let (left, right) = options.split_once(", ").unwrap();
            nodes.0.insert(name.into(), [left.into(), right.into()]);
        }
        Ok(nodes)
    }
}

pub struct Input {
    pub instructions: Directions,
    pub nodes: Nodes,
}

impl Input {
    fn step(&self, node: &str, dir: Direction) -> Option<&str> {
        let d = self.nodes.0.get(node)?;
        let n = match dir {
            Direction::Left => &d[0],
            Direction::Right => &d[1],
        };
        Some(n)
    }

    fn sequence<'a>(&'a self, start: &'a str) -> impl Iterator<Item = &'a str> {
        self.instructions
            .0
            .iter()
            .cycle()
            .scan(start, |state, dir| {
                *state = self.step(*state, *dir).unwrap();
                Some(*state)
            })
    }

    fn solve(&self) -> usize {
        self.sequence("AAA")
            .take_while(|state| state != &"ZZZ")
            .count()
            + 1
    }
}

impl FromStr for Input {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (header, body) = input.split_once("\n\n").unwrap();
        Ok(Input {
            instructions: header.parse().unwrap(),
            nodes: body.parse().unwrap(),
        })
    }
}

#[test]
fn can_parse() {
    let _input1: Input = include_str!("../sample1.txt").parse().unwrap();
    let _input1: Input = include_str!("../sample2.txt").parse().unwrap();
}

#[test]
fn can_solve() {
    let input1: Input = include_str!("../sample1.txt").parse().unwrap();
    assert_eq!(input1.solve(), 2);

    let input2: Input = include_str!("../sample2.txt").parse().unwrap();
    assert_eq!(input2.solve(), 6);
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let input: Input = input.parse().unwrap();
    println!("sum {}", input.solve());
}
