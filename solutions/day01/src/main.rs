use std::{io::*, iter::once};

/// Find the first and last digit in a string, and turn them into a number.
fn solve(input: &str) -> u32 {
    let mut numbers = input.chars().flat_map(|c| c.to_digit(10));
    let first = numbers.next().unwrap();
    let last = numbers.last().unwrap_or(first);
    let number = 10 * first + last;
    number
}

fn prefixes(s: &str) -> impl Iterator<Item = &str> + DoubleEndedIterator {
    s.char_indices()
        .map(move |(pos, _)| &s[..pos])
        .chain(once(s))
}

fn suffixes(s: &str) -> impl Iterator<Item = &str> + DoubleEndedIterator {
    s.char_indices()
        .map(move |(pos, _)| &s[pos..])
        .chain(once(""))
        .rev()
}

/// Find the first and last digit in a string, but the digit may also be written
/// out as text.
fn solve2(input: &str) -> u32 {
    let digits: [&[&'static str]; 10] = [
        &["0"],
        &["one", "1"],
        &["two", "2"],
        &["three", "3"],
        &["four", "4"],
        &["five", "5"],
        &["six", "6"],
        &["seven", "7"],
        &["eight", "8"],
        &["nine", "9"],
    ];

    let first = suffixes(input)
        .rev()
        .flat_map(|prefix| {
            digits
                .iter()
                .enumerate()
                .find(|(_, patterns)| patterns.iter().any(|pattern| prefix.starts_with(pattern)))
                .map(|(index, _)| index)
        })
        .next()
        .unwrap();

    let last = prefixes(input)
        .rev()
        .flat_map(|prefix| {
            digits
                .iter()
                .enumerate()
                .find(|(_, patterns)| patterns.iter().any(|pattern| prefix.ends_with(pattern)))
                .map(|(index, _)| index)
        })
        .next()
        .unwrap();

    (10 * first + last) as u32
}

fn main() {
    let input = BufReader::new(stdin());
    let solution: u32 = input.lines().map(|line| solve2(&line.unwrap())).sum();
    println!("{solution}");
}

#[test]
fn can_solve_sample() {
    assert_eq!(solve("1abc2"), 12);
    assert_eq!(solve("pqr3stu8vwx"), 38);
    assert_eq!(solve("a1b2c3d4e5f"), 15);
    assert_eq!(solve("treb7uchet"), 77);
}

#[test]
fn can_solve2() {
    assert_eq!(solve2("two1nine"), 29);
    assert_eq!(solve2("eightwothree"), 83);
    assert_eq!(solve2("abcone2threexyz"), 13);
    assert_eq!(solve2("xtwone3four"), 24);
    assert_eq!(solve2("4nineeightseven2"), 42);
    assert_eq!(solve2("zoneight234"), 14);
    assert_eq!(solve2("7pqrstsixteen"), 76);
}
