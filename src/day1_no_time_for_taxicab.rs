use std::collections::HashSet;
use std::str::FromStr;

pub(crate) fn run() {
    let _input = "R5, L5, R5, R3";
    let _input = "R8, R4, R4, R8";
    let _input = _get_input();

    let instructions: Vec<Instruction> = _input.split_whitespace().map(|line| line.trim_matches(',').parse().unwrap()).collect();

    let mut person = Person::new();
    for instruction in instructions.iter() {
        person.walk(instruction);
    }
    println!("last destination is {} blocks away", person.distance_from_start());

    let mut person = Person::new();
    let mut locations_visited: HashSet<Coord> = Default::default();
    for instruction in instructions.iter() {
        let path = person.walk(instruction);
        let first_location_visited_twice = path.iter().filter(|c| locations_visited.contains(c))
            .next();
        if let Some(location) = first_location_visited_twice {
            println!("first location visited twice is {} blocks away", location.get_manhattan_distance(&Default::default()));
            return;
        }
        locations_visited.extend(path.into_iter());
    }
    println!("no locations were visited twice");
}

#[derive(Debug)]
enum Error {
    ParseInstructionError(&'static str),
}

struct Person {
    facing_direction: FacingDirection,
    position: Coord,
}

impl Person {
    pub(crate) fn new() -> Self {
        Self { facing_direction: FacingDirection::North, position: Default::default() }
    }
    pub(crate) fn walk(&mut self, instruction: &Instruction) -> Vec<Coord> {
        self.facing_direction = self.facing_direction.turn(&instruction.turn_direction);
        let path = self.position.get_next(&self.facing_direction, instruction.walk_blocks);
        if let Some(last) = path.last() {
            self.position = last.clone();
        }
        path
    }
    pub(crate) fn distance_from_start(&self) -> i32 {
        self.position.get_manhattan_distance(&Default::default())
    }
}

enum TurnDirection {
    Left,
    Right,
}

struct Instruction {
    turn_direction: TurnDirection,
    walk_blocks: i32,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            turn_direction: match s.chars().nth(0).ok_or(Error::ParseInstructionError("cannot get direction"))? {
                'L' => TurnDirection::Left,
                'R' => TurnDirection::Right,
                _ => return Err(Error::ParseInstructionError("invalid direction"))
            },
            walk_blocks: s[1..].parse().or(Err(Error::ParseInstructionError("invalid walk_blocks")))?,
        })
    }
}

enum FacingDirection {
    North,
    South,
    East,
    West,
}

impl FacingDirection {
    pub(crate) fn turn(&self, direction: &TurnDirection) -> Self {
        match self {
            FacingDirection::North => match direction {
                TurnDirection::Left => Self::West,
                TurnDirection::Right => Self::East
            }
            FacingDirection::South => match direction {
                TurnDirection::Left => Self::East,
                TurnDirection::Right => Self::West
            }
            FacingDirection::East => match direction {
                TurnDirection::Left => Self::North,
                TurnDirection::Right => Self::South
            }
            FacingDirection::West => match direction {
                TurnDirection::Left => Self::South,
                TurnDirection::Right => Self::North
            }
        }
    }
}

#[derive(Default, Clone, Hash, Eq, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn get_axis(&mut self, direction: &FacingDirection) -> &mut i32 {
        match direction {
            FacingDirection::North | FacingDirection::South => {
                &mut self.y
            }
            FacingDirection::East | FacingDirection::West => {
                &mut self.x
            }
        }
    }
    pub(crate) fn get_next(&self, direction: &FacingDirection, blocks: i32) -> Vec<Self> {
        let mut result = self.clone();
        let sign: i32 = match direction {
            FacingDirection::North | FacingDirection::East => 1,
            FacingDirection::South | FacingDirection::West => -1
        };

        let mut results: Vec<Self> = Default::default();
        for _ in 0..blocks {
            *result.get_axis(direction) += sign;
            results.push(result.clone());
        }

        results
    }
    pub(crate) fn get_manhattan_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() +
            (self.y - other.y).abs()
    }
}

fn _get_input() -> &'static str {
    "R4, R1, L2, R1, L1, L1, R1, L5, R1, R5, L2, R3, L3, L4, R4, R4, R3, L5, L1, R5, R3, L4, R1, R5, L1, R3, L2, R3, R1, L4, L1, R1, L1, L5, R1, L2, R2, L3, L5, R1, R5, L1, R188, L3, R2, R52, R5, L3, R79, L1, R5, R186, R2, R1, L3, L5, L2, R2, R4, R5, R5, L5, L4, R5, R3, L4, R4, L4, L4, R5, L4, L3, L1, L4, R1, R2, L5, R3, L4, R3, L3, L5, R1, R1, L3, R2, R1, R2, R2, L4, R5, R1, R3, R2, L2, L2, L1, R2, L1, L3, R5, R1, R4, R5, R2, R2, R4, R4, R1, L3, R4, L2, R2, R1, R3, L5, R5, R2, R5, L1, R2, R4, L1, R5, L3, L3, R1, L4, R2, L2, R1, L1, R4, R3, L2, L3, R3, L2, R1, L4, R5, L1, R5, L2, L1, L5, L2, L5, L2, L4, L2, R3"
}