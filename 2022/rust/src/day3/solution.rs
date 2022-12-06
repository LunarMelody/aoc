use std::collections::{HashMap, HashSet};

use super::input::INPUT_DATA;

pub fn get_char_priorities() -> HashMap<char, u8> {
    ('a'..='z').into_iter().chain('A'..='Z').zip(1..).collect()
}

pub fn part1() {
    let priorities = get_char_priorities();

    let rucsacks: Vec<(&str, &str)> = INPUT_DATA
        .lines()
        .map(|items| {
            let per_compartment = items.len().wrapping_div(2);

            items.split_at(per_compartment)
        })
        .collect();

    let total_priorities: u32 = rucsacks
        .iter()
        .map(|(compartment1, compartment2)| {
            let comp1: HashSet<char> = compartment1.chars().collect();
            let comp2: HashSet<char> = compartment2.chars().collect();

            let intersection = comp1.intersection(&comp2).nth(0);
            match intersection {
                Some(v) => *priorities.get(v).unwrap() as u32,
                None => 0,
            }
        })
        .sum();

    println!("{total_priorities}");
}

pub fn part2() {
    let priorities = get_char_priorities();

    let rucksacks: Vec<&str> = INPUT_DATA.lines().collect();

    let total_priorities: u32 = rucksacks
        .chunks(3)
        .map(|group| {
            let first: HashSet<_> = group.get(0).unwrap().chars().collect();
            let second: HashSet<_> = group.get(1).unwrap().chars().collect();
            let third: HashSet<_> = group.get(2).unwrap().chars().collect();

            let first_intersection = first
                .intersection(&second)
                .map(|entry| *entry)
                .collect::<HashSet<_>>();

            let full_intersection = first_intersection.intersection(&third).nth(0).unwrap();

            *priorities
                .get(full_intersection)
                .expect("You can't get a priority for something that doesnt have score")
                as u32
        })
        .sum();

    println!("{total_priorities}");
}
