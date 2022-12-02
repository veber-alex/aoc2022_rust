enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl From<u8> for Shape {
    fn from(value: u8) -> Self {
        match value {
            b'A' | b'X' => Rock,
            b'B' | b'Y' => Paper,
            b'C' | b'Z' => Scissors,
            _ => unreachable!(),
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl From<u8> for Outcome {
    fn from(value: u8) -> Self {
        match value {
            b'X' => Lose,
            b'Y' => Draw,
            b'Z' => Win,
            _ => unreachable!(),
        }
    }
}

use Outcome::*;
use Shape::*;

fn main() {
    let input = include_str!("../input/day2.txt");

    let mut score = 0;
    for line in input.as_bytes().chunks(4) {
        let (other, me) = (line[0], line[2]);
        score += match (Shape::from(me), Shape::from(other)) {
            (Rock, Scissors) => 1 + 6,
            (Rock, Rock) => 1 + 3,
            (Rock, Paper) => 1 + 0,

            (Paper, Rock) => 2 + 6,
            (Paper, Paper) => 2 + 3,
            (Paper, Scissors) => 2 + 0,

            (Scissors, Paper) => 3 + 6,
            (Scissors, Scissors) => 3 + 3,
            (Scissors, Rock) => 3 + 0,
        };
    }

    // part 1
    assert_eq!(score, 14827);

    let mut score = 0;
    for line in input.as_bytes().chunks(4) {
        let (other, outcome) = (line[0], line[2]);
        score += match (Shape::from(other), Outcome::from(outcome)) {
            (Rock, Win) => 2 + 6,
            (Rock, Draw) => 1 + 3,
            (Rock, Lose) => 3 + 0,

            (Paper, Win) => 3 + 6,
            (Paper, Draw) => 2 + 3,
            (Paper, Lose) => 1 + 0,

            (Scissors, Win) => 1 + 6,
            (Scissors, Draw) => 3 + 3,
            (Scissors, Lose) => 2 + 0,
        };
    }

    // part 2
    assert_eq!(score, 13889);
}
