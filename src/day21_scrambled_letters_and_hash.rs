use std::fmt::{Debug, Formatter};
use std::str::{FromStr};
use anyhow::Result;
use log::debug;

pub(crate) fn run() {
    let _input = "swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";
    let _input = _get_input();

    let _pass = "abcde";
    let _pass = "abcdefgh";

    let mut password = Password::new(_pass);

    let instructions: Vec<Instruction> = _input.lines().map(|line| line.parse().unwrap()).collect();
    println!("original password: {:?}", password);
    for instruction in instructions.iter() {
        password.scramble(instruction).unwrap();
        debug!("scrambling: {:?} -> {:?}", instruction, password);
    }

    println!("scrambled password: {:?}", password);

    let _pass = "decab";
    let _pass = "fbgdceah";
    let mut password = Password::new(_pass);

    println!("scrambled password: {:?}", password);
    for instruction in instructions.iter().rev() {
        password.unscramble(instruction).unwrap();
        debug!("unscrambling: {:?} -> {:?}", instruction, password);
    }

    println!("unscrambled password: {:?}", password);
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("parse error: {}", .0)]
    ParseError(String),
    #[error("scrambling error")]
    ScramblingError,
    #[error("unscrambling error")]
    UnscramblingError,
}

struct Password(Vec<Character>);

impl Debug for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|c| format!("{:?}", c)).collect::<String>())
    }
}

impl Password {
    pub(crate) fn new(s: &str) -> Self {
        Self(s.chars().map(|c| c.into()).collect())
    }
    fn swap_positions(&mut self, a: usize, b: usize) {
        let mut temp = self.0[b];
        std::mem::swap(&mut self.0[a], &mut temp);
        std::mem::swap(&mut self.0[b], &mut temp);
    }
    fn rotate_right(&mut self, amount: isize) {
        let reference = self.0.clone();
        for (i, val) in self.0.iter_mut().enumerate() {
            let j = (((i as isize - amount) % reference.len() as isize) + reference.len() as isize) as usize % reference.len();
            *val = reference[j];
        }
    }
    pub(crate) fn scramble(&mut self, instruction: &Instruction) -> Result<()> {
        let position = |letter: &Character| {
            self.0.iter().position(|c| c == letter).ok_or(Error::ScramblingError)
        };
        match instruction {
            Instruction::SwapPositions(a, b) => {
                self.swap_positions(*a, *b);
            }
            Instruction::SwapLetters(c_a, c_b) => {
                let a = position(c_a)?;
                let b = position(c_b)?;
                self.swap_positions(a, b);
            }
            Instruction::Rotate(dir, amount) => {
                let amount = *amount as isize * match dir {
                    Direction::Left => -1,
                    Direction::Right => 1
                };
                self.rotate_right(amount);
            }
            Instruction::RotateRightBasedOn(letter) => {
                let index = position(letter)?;
                let amount = 1 + index + if index >= 4 { 1 } else { 0 };
                self.rotate_right(amount as isize);
            }
            Instruction::ReversePositionsInclusive { from, to } => {
                let mut slice = self.0[*from..=*to].to_vec();
                slice.reverse();
                for (i, val) in self.0[*from..=*to].iter_mut().enumerate() {
                    *val = slice[i];
                }
            }
            Instruction::Move { from, to } => {
                let val = self.0.remove(*from);
                self.0.insert(*to, val);
            }
        }
        Ok(())
    }
    pub(crate) fn unscramble(&mut self, instruction: &Instruction) -> Result<()> {
        let position = |letter: &Character| {
            self.0.iter().position(|c| c == letter).ok_or(Error::ScramblingError)
        };
        match instruction {
            Instruction::SwapPositions(_, _) | Instruction::SwapLetters(_, _) | Instruction::ReversePositionsInclusive { .. } => {
                self.scramble(instruction)?;
            }
            Instruction::Rotate(dir, amount) => {
                let amount = *amount as isize * match dir {
                    Direction::Left => -1,
                    Direction::Right => 1
                };
                self.rotate_right(-amount);
            }
            Instruction::RotateRightBasedOn(letter) => {
                let map = |index: usize| {
                    (index + 1 + index + if index >= 4 { 1 } else { 0 }) % self.0.len()
                };
                let target = position(letter)?;
                let original = (0..self.0.len()).filter(|&i| map(i) == target)
                    .next().ok_or(Error::UnscramblingError)?;
                debug!("original was {}, new target is {}", original, target);
                let amount = (original + self.0.len() - target) % self.0.len();
                self.rotate_right(amount as isize);
            }
            Instruction::Move { from: to, to: from } => {
                let val = self.0.remove(*from);
                self.0.insert(*to, val);
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Instruction {
    SwapPositions(usize, usize),
    SwapLetters(Character, Character),
    Rotate(Direction, usize),
    RotateRightBasedOn(Character),
    ReversePositionsInclusive { from: usize, to: usize },
    Move { from: usize, to: usize },
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let mut next = || {
            words.next().ok_or(Error::ParseError(s.to_string()))
        };
        let last = |words: std::str::SplitWhitespace| {
            words.last().map(|w| w.to_owned()).ok_or(Error::ParseError(s.to_string()))
        };
        Ok(
            match next()? {
                "swap" => {
                    match next()? {
                        "position" => {
                            let left: usize = next()?.parse()?;
                            let right: usize = last(words)?.parse()?;
                            Self::SwapPositions(left, right)
                        }
                        "letter" => {
                            let left: Character = next()?.parse()?;
                            let right: Character = last(words)?.parse()?;
                            Self::SwapLetters(left, right)
                        }
                        _ => return Err(Error::ParseError(s.to_string()).into()),
                    }
                }
                "rotate" => {
                    match next()? {
                        "left" => {
                            let direction = Direction::Left;
                            let amount: usize = next()?.parse()?;
                            Self::Rotate(direction, amount)
                        }
                        "right" => {
                            let direction = Direction::Right;
                            let amount: usize = next()?.parse()?;
                            Self::Rotate(direction, amount)
                        }
                        "based" => {
                            let letter: Character = last(words)?.parse()?;
                            Self::RotateRightBasedOn(letter)
                        }
                        _ => return Err(Error::ParseError(s.to_string()).into())
                    }
                }
                "reverse" => {
                    next()?;
                    let from: usize = next()?.parse()?;
                    next()?;
                    let to: usize = next()?.parse()?;
                    Self::ReversePositionsInclusive { from, to }
                }
                "move" => {
                    next()?;
                    let from: usize = next()?.parse()?;
                    next()?;
                    next()?;
                    let to: usize = next()?.parse()?;
                    Self::Move { from, to }
                }
                _ => return Err(Error::ParseError(s.to_string()).into()),
            }
        )
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Default, Copy, Clone, PartialEq)]
struct Character(u8);

impl Debug for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.0 + 'a' as u8) as char)
    }
}

impl FromStr for Character {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().skip(1).any(|_| true) {
            Err(Error::ParseError(s.to_string()).into())
        } else {
            Ok(s.chars().next().ok_or(Error::ParseError(s.to_string()))?.into())
        }
    }
}

impl From<char> for Character {
    fn from(c: char) -> Self {
        Self(c as u8 - 'a' as u8)
    }
}

impl From<Character> for char {
    fn from(c: Character) -> Self {
        (c.0 + 'a' as u8) as char
    }
}

fn _get_input() -> &'static str {
    "swap letter a with letter d
move position 6 to position 4
move position 5 to position 1
swap letter h with letter e
rotate based on position of letter a
move position 6 to position 2
reverse positions 0 through 1
rotate based on position of letter h
rotate based on position of letter g
rotate based on position of letter h
reverse positions 4 through 7
swap letter a with letter f
swap position 2 with position 7
move position 7 to position 5
reverse positions 0 through 5
rotate based on position of letter f
rotate right 4 steps
swap position 3 with position 0
move position 1 to position 2
reverse positions 4 through 6
swap position 3 with position 5
swap letter a with letter c
swap position 5 with position 2
swap position 7 with position 2
move position 2 to position 5
rotate based on position of letter h
rotate right 2 steps
swap position 3 with position 4
move position 0 to position 1
reverse positions 1 through 7
reverse positions 1 through 4
rotate based on position of letter b
rotate right 7 steps
rotate left 0 steps
swap position 6 with position 1
reverse positions 1 through 3
reverse positions 0 through 3
move position 0 to position 4
rotate based on position of letter f
reverse positions 0 through 7
reverse positions 0 through 1
move position 1 to position 7
move position 7 to position 6
rotate based on position of letter b
reverse positions 3 through 5
reverse positions 0 through 3
swap letter c with letter h
reverse positions 3 through 5
swap position 3 with position 6
swap letter d with letter g
move position 5 to position 6
swap position 6 with position 2
rotate right 5 steps
swap letter e with letter g
rotate based on position of letter e
rotate based on position of letter c
swap letter g with letter e
rotate based on position of letter b
rotate based on position of letter b
swap position 0 with position 2
move position 6 to position 0
move position 5 to position 0
rotate left 2 steps
move position 0 to position 5
rotate left 7 steps
swap letter b with letter g
rotate based on position of letter d
swap letter h with letter e
swap letter d with letter c
rotate based on position of letter f
move position 5 to position 0
rotate left 5 steps
swap position 0 with position 7
swap position 0 with position 3
rotate left 4 steps
rotate left 1 step
rotate right 6 steps
swap position 0 with position 1
reverse positions 4 through 6
reverse positions 4 through 6
move position 6 to position 3
move position 7 to position 4
rotate right 4 steps
swap letter g with letter d
swap letter c with letter e
swap letter e with letter h
rotate right 5 steps
rotate based on position of letter g
rotate based on position of letter g
rotate left 3 steps
swap letter h with letter g
reverse positions 0 through 4
rotate right 4 steps
move position 6 to position 4
rotate based on position of letter c
swap position 2 with position 6
swap position 7 with position 2
rotate right 1 step
swap position 3 with position 1
swap position 4 with position 6"
}