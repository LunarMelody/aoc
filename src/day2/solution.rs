use std::{
    cmp::Ordering::{self, Equal, Greater, Less},
    str::FromStr,
};

use super::input::INPUT_DATA;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Outcome {
    LOSS = 0,
    DRAW = 3,
    VICTORY = 6,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::LOSS),
            "Y" => Ok(Outcome::DRAW),
            "Z" => Ok(Outcome::VICTORY),
            _ => panic!("Bad Outcome code: expected X Y Z, received {}", s),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Weapon {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

impl Weapon {
    fn from_wanted_outcome(outcome: &Outcome, enemy: &Weapon) -> Self {
        match (enemy, outcome) {
            (Weapon::ROCK, Outcome::LOSS)
            | (Weapon::PAPER, Outcome::VICTORY)
            | (Weapon::SCISSORS, Outcome::DRAW) => Weapon::SCISSORS,

            (Weapon::ROCK, Outcome::DRAW)
            | (Weapon::SCISSORS, Outcome::VICTORY)
            | (Weapon::PAPER, Outcome::LOSS) => Weapon::ROCK,

            (Weapon::ROCK, Outcome::VICTORY)
            | (Weapon::PAPER, Outcome::DRAW)
            | (Weapon::SCISSORS, Outcome::LOSS) => Weapon::PAPER,
        }
    }
}

impl FromStr for Weapon {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Weapon::ROCK),
            "B" | "Y" => Ok(Weapon::PAPER),
            "C" | "Z" => Ok(Weapon::SCISSORS),
            _ => panic!(
                "Bad weapon code provided, expected A B C or X Y Z, received {}",
                s
            ),
        }
    }
}

impl PartialOrd for Weapon {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Weapon::ROCK, Weapon::ROCK)
            | (Weapon::PAPER, Weapon::PAPER)
            | (Weapon::SCISSORS, Weapon::SCISSORS) => Some(Ordering::Equal),

            (Weapon::ROCK, Weapon::PAPER)
            | (Weapon::PAPER, Weapon::SCISSORS)
            | (Weapon::SCISSORS, Weapon::ROCK) => Some(Ordering::Less),

            (Weapon::ROCK, Weapon::SCISSORS)
            | (Weapon::PAPER, Weapon::ROCK)
            | (Weapon::SCISSORS, Weapon::PAPER) => Some(Ordering::Greater),
        }
    }
}

impl Ord for Weapon {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[allow(dead_code)]
pub fn part1() {
    let lines = INPUT_DATA.lines();
    let mut my_total_score: i32 = 0;

    lines.for_each(|line| {
        let my_weapon: Weapon = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        let enemy_weapon: Weapon = line.split_whitespace().nth(0).unwrap().parse().unwrap();

        let outcome = match my_weapon.cmp(&enemy_weapon) {
            Less => Outcome::LOSS,
            Equal => Outcome::DRAW,
            Greater => Outcome::VICTORY,
        };

        my_total_score = my_total_score + my_weapon as i32 + outcome as i32;
    });

    println!("Total score: {}", my_total_score);
}

#[allow(dead_code)]
pub fn part2() {
    let lines = INPUT_DATA.lines();
    let mut my_total_score: i32 = 0;

    lines.for_each(|line| {
        let enemy_weapon = line
            .split_whitespace()
            .nth(0)
            .unwrap()
            .parse::<Weapon>()
            .unwrap();

        let my_wanted_outcome = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<Outcome>()
            .unwrap();

        let my_wanted_weapon = Weapon::from_wanted_outcome(&my_wanted_outcome, &enemy_weapon);

        my_total_score = my_total_score + my_wanted_outcome as i32 + my_wanted_weapon as i32;
    });

    println!("Total score: {}", my_total_score);
}
