use itertools::Itertools;
use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

/// Point as (x, y)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct Point<T = usize>(T, T);

impl Point {
    fn touching(&self) -> impl Iterator<Item = Point> {
        let y = [0isize, 1, 1, 1, 0, -1, -1, -1];
        //let x = [0, 1, 1, 1, 0, -1, -1, -1];

        let point = *self;
        y.into_iter()
            .zip(y.into_iter().cycle().skip(6))
            .flat_map(move |(x, y)| {
                let x = point.0.checked_add_signed(x)?;
                let y = point.1.checked_add_signed(y)?;
                Some(Point(x, y))
            })
    }
}

/// Entity that can be on the schematic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Entity {
    /// Symbol
    Symbol(char),
    /// Number (index into numbers array)
    Number(usize),
}

#[derive(Debug, Clone, Default)]
pub struct Schematic {
    numbers: Vec<u64>,
    field: BTreeMap<Point, Entity>,
}

impl Schematic {
    fn get<T: TryInto<usize>>(&self, point: Point<T>) -> Option<Entity> {
        let point = Point(point.0.try_into().ok()?, point.1.try_into().ok()?);
        self.field.get(&point).copied()
    }

    fn get_symbol<T: TryInto<usize>>(&self, point: Point<T>) -> Option<char> {
        match self.get(point) {
            Some(Entity::Symbol(s)) => Some(s),
            _ => None,
        }
    }

    fn get_number<T: TryInto<usize>>(&self, point: Point<T>) -> Option<u64> {
        match self.get(point) {
            Some(Entity::Number(o)) => self.numbers.get(o).copied(),
            _ => None,
        }
    }

    fn part_number_offsets(&self) -> BTreeSet<usize> {
        let mut set = BTreeSet::default();
        for (pos, entity) in self.field.iter() {
            let index = match entity {
                Entity::Number(offset) => offset,
                _ => continue,
            };

            let touching = pos.touching().any(|pos| self.get_symbol(pos).is_some());

            if touching {
                set.insert(*index);
            }
        }

        set
    }

    fn part_numbers(&self) -> Vec<u64> {
        self.part_number_offsets()
            .into_iter()
            .flat_map(|offset| self.numbers.get(offset))
            .copied()
            .collect()
    }
}

impl FromStr for Schematic {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut schematic = Schematic::default();
        for (y, line) in input.lines().enumerate() {
            // insert symbols
            for (x, c) in line.chars().enumerate() {
                if !matches!(c, '.' | '0'..='9') {
                    schematic.field.insert(Point(x, y), Entity::Symbol(c));
                }
            }

            let numbers = line
                .match_indices(|c| matches!(c, '0'..='9'))
                .peekable()
                .batching(|it| {
                    let (start, first) = it.next()?;
                    let mut range = start..start + first.len();
                    let mut slice = first;
                    while let Some((cur, _)) = it.peek() {
                        if *cur == range.end {
                            let (_cur, next) = it.next().unwrap();
                            range.end += next.len();
                            slice = unsafe { slice.get_unchecked(0..slice.len() + next.len()) };
                        } else {
                            break;
                        }
                    }

                    Some((range, slice))
                });

            // insert numbers
            for (range, number) in numbers {
                let number: u64 = number.parse().unwrap();
                schematic.numbers.push(number);
                let number = Entity::Number(schematic.numbers.len() - 1);
                for x in range {
                    schematic.field.insert(Point(x, y), number);
                }
            }
        }

        Ok(schematic)
    }
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let schematic: Schematic = input.parse().unwrap();
    let sum: u64 = schematic.part_numbers().into_iter().sum();
    println!("{sum}");
}

#[test]
fn can_parse() {
    let input = include_str!("input1.txt");
    let schematic: Schematic = input.parse().unwrap();
    use Entity::*;

    println!("{schematic:?}");
    assert_eq!(
        schematic.numbers,
        [467, 114, 35, 633, 617, 58, 592, 755, 664, 598,]
    );

    // symbols
    assert_eq!(schematic.get_symbol(Point(3, 1)), Some('*'));
    assert_eq!(schematic.get_symbol(Point(6, 3)), Some('#'));
    assert_eq!(schematic.get_symbol(Point(3, 4)), Some('*'));
    assert_eq!(schematic.get_symbol(Point(5, 5)), Some('+'));
    assert_eq!(schematic.get_symbol(Point(3, 8)), Some('$'));
    assert_eq!(schematic.get_symbol(Point(5, 8)), Some('*'));

    // numbers
    assert_eq!(schematic.get_number(Point(0, 0)), Some(467));
    assert_eq!(schematic.get_number(Point(1, 0)), Some(467));
    assert_eq!(schematic.get_number(Point(2, 0)), Some(467));
    assert_eq!(schematic.get_number(Point(5, 0)), Some(114));
    assert_eq!(schematic.get_number(Point(6, 0)), Some(114));
    assert_eq!(schematic.get_number(Point(7, 0)), Some(114));
    assert_eq!(schematic.get_number(Point(2, 2)), Some(35));
    assert_eq!(schematic.get_number(Point(3, 2)), Some(35));
}

#[test]
fn can_solve() {
    let input = include_str!("input1.txt");
    let schematic: Schematic = input.parse().unwrap();
    use Entity::*;

    let numbers = schematic.part_numbers();
    assert_eq!(numbers, [467, 35, 633, 617, 592, 755, 664, 598]);
}
