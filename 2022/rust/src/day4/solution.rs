use std::{cmp::Ordering, ops::RangeInclusive, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
struct SectionRange {
    start: u16,
    end: u16,
}

impl SectionRange {
    fn as_range(&self) -> RangeInclusive<u16> {
        self.start..=self.end
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Pair {
    first: SectionRange,
    second: SectionRange,
}

impl Pair {
    fn bigger_smaller_ranges(&self) -> (RangeInclusive<u16>, RangeInclusive<u16>) {
        let first_range = self.first.as_range();
        let second_range = self.second.as_range();

        match first_range.len().cmp(&second_range.len()) {
            Ordering::Greater => (first_range, second_range),
            _ => (second_range, first_range),
        }
    }

    fn has_full_overlap(&self) -> bool {
        let (bigger, mut smaller) = self.bigger_smaller_ranges();
        smaller.all(|n| bigger.contains(&n))
    }

    fn has_overlap(&self) -> bool {
        let (bigger, mut smaller) = self.bigger_smaller_ranges();
        smaller.any(|n| bigger.contains(&n))
    }
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_range, second_range) = s
            .split_once(",")
            .expect("Pair string must be separated with comma");

        let (first_start, first_end) = first_range
            .split_once("-")
            .expect("Pair's first section range must be separated with a dash");

        let (second_start, second_end) = second_range
            .split_once("-")
            .expect("Pair's second section range must be separated with a dash");

        let (first_start, first_end): (u16, u16) =
            (first_start.parse().unwrap(), first_end.parse().unwrap());

        let (second_start, second_end): (u16, u16) =
            (second_start.parse().unwrap(), second_end.parse().unwrap());

        Ok(Pair {
            first: SectionRange {
                start: first_start,
                end: first_end,
            },
            second: SectionRange {
                start: second_start,
                end: second_end,
            },
        })
    }
}

fn get_pairs() -> Vec<Pair> {
    super::input::INPUT_DATA
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn part1() {
    let pairs = get_pairs();
    let pairs_to_reconsider = pairs.iter().fold(0, |mut acc, pair| {
        if pair.has_full_overlap() {
            acc = acc + 1;
        }

        acc
    });

    println!("{pairs_to_reconsider}");
}

pub fn part2() {
    let pairs = get_pairs();
    let pairs_to_actually_reconsider = pairs.iter().fold(0, |mut acc, pair| {
        if pair.has_overlap() {
            acc = acc + 1
        }

        acc
    });

    println!("{pairs_to_actually_reconsider}")
}
