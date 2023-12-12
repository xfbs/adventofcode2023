use std::{collections::BTreeMap, str::FromStr};

#[derive(Debug, Clone)]
pub struct MapRange {
    pub source: u64,
    pub destination: u64,
    pub length: u64,
}

impl FromStr for MapRange {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut numbers = input.split_whitespace().map(|n| n.parse().unwrap());
        Ok(MapRange {
            destination: numbers.next().unwrap(),
            source: numbers.next().unwrap(),
            length: numbers.next().unwrap(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    pub source: String,
    pub destination: String,
    pub mapping: BTreeMap<u64, MapRange>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines();
        let header = lines.next().unwrap();
        let header = header.split_whitespace().next().unwrap();
        let mut header = header.split("-");
        let source = header.next().unwrap().to_string();
        let destination = header.nth(1).unwrap().to_string();
        let map = Map {
            source,
            destination,
            mapping: Default::default(),
        };
        let map = lines
            .map(|line| line.parse().unwrap())
            .fold(map, |mut map, range| {
                map.add(range);
                map
            });
        Ok(map)
    }
}

impl Map {
    pub fn add(&mut self, map: MapRange) {
        if self.try_map(map.source).is_some() {
            panic!("Mapping exists");
        }

        if self
            .mapping
            .range(map.source..map.source + map.length)
            .next()
            .is_some()
        {
            panic!("Mapping exists");
        }

        self.mapping.insert(map.source, map);
    }

    pub fn try_map(&self, value: u64) -> Option<u64> {
        self.mapping
            .range(..=value)
            .rev()
            .next()
            .and_then(|(_, map)| {
                let offset = value - map.source;
                (offset < map.length).then(|| map.destination + offset)
            })
    }

    pub fn map(&self, value: u64) -> u64 {
        self.try_map(value).unwrap_or(value)
    }
}

#[test]
fn can_map() {
    let mut map = Map {
        source: "seed".into(),
        destination: "soil".into(),
        mapping: Default::default(),
    };
    map.add(MapRange {
        source: 98,
        destination: 50,
        length: 2,
    });
    map.add(MapRange {
        source: 50,
        destination: 52,
        length: 48,
    });

    for i in 0..50 {
        assert_eq!(map.map(i), i);
    }

    for i in 50..98 {
        assert_eq!(map.map(i), i + 2);
    }

    for i in 98..100 {
        assert_eq!(map.map(i), i - 48);
    }

    for i in 100..200 {
        assert_eq!(map.map(i), i);
    }
}

#[derive(Debug, Clone)]
pub struct Alamanac {
    pub seeds: Vec<u64>,
    pub maps: BTreeMap<String, Map>,
}

impl Alamanac {
    fn map_once(&self, source: &str, value: u64) -> Option<(&str, u64)> {
        self.maps
            .get(source)
            .map(|map| (map.destination.as_str(), map.map(value)))
    }

    fn map_full<'a, 'b: 'a>(&'a self, source: &'b str, value: u64) -> (&'a str, u64) {
        let mut current = (source, value);
        while let Some((source, value)) = self.map_once(current.0, current.1) {
            current = (source, value);
        }
        current
    }
}

impl FromStr for Alamanac {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut blocks = input.split("\n\n");
        let seeds = blocks.next().unwrap();

        Ok(Alamanac {
            seeds: seeds
                .split_whitespace()
                .skip(1)
                .map(|s| s.parse().unwrap())
                .collect(),
            maps: blocks
                .map(|block| block.parse().unwrap())
                .map(|map: Map| (map.source.clone(), map))
                .collect(),
        })
    }
}

#[test]
fn can_parse() {
    let input = include_str!("input.txt");
    let alamanac: Alamanac = input.parse().unwrap();
    assert_eq!(alamanac.seeds, [79, 14, 55, 13]);
}

#[test]
fn can_solve() {
    let input = include_str!("input.txt");
    let alamanac: Alamanac = input.parse().unwrap();
    assert_eq!(alamanac.map_once("seed", 79), Some(("soil", 81)));
    assert_eq!(alamanac.map_once("seed", 14), Some(("soil", 14)));
    assert_eq!(alamanac.map_once("seed", 55), Some(("soil", 57)));
    assert_eq!(alamanac.map_once("seed", 13), Some(("soil", 13)));

    assert_eq!(alamanac.map_full("seed", 79), ("location", 82));
    assert_eq!(alamanac.map_full("seed", 14), ("location", 43));
    assert_eq!(alamanac.map_full("seed", 55), ("location", 86));
    assert_eq!(alamanac.map_full("seed", 13), ("location", 35));
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let alamanac: Alamanac = input.parse().unwrap();
    let min: u64 = alamanac
        .seeds
        .iter()
        .map(|seed| alamanac.map_full("seed", *seed).1)
        .min()
        .unwrap();
    println!("min_location {min}");
}
