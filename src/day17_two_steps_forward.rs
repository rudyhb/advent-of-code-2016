use std::collections::HashSet;
use utils::a_star::{a_star_search, AStarNode, CurrentNodeDetails, Successor};
use lazy_static::lazy_static;

pub(crate) fn run() {
    let _input = "ihgpwlah";
    let _input = "kglvqrro";
    let _input = "ulqzkmiv";
    let _input = "udskfozm";

    let start = Path {
        current_position: Coord { x: 0, y: 0 },
        history: vec![],
        passcode: _input,
    };
    let end = Path {
        current_position: END_POSITION.clone(),
        history: vec![],
        passcode: _input,
    };
    let result = a_star_search(start, &end, get_successors, distance_function, None).unwrap();
    println!("the shortest path is {}", result.iter().last().unwrap().history.iter().collect::<String>());

    println!("the longest path is {}", get_longest_path(_input).len());
}

lazy_static! {
    static ref END_POSITION: Coord = Coord {x: 3, y: 3};
}

fn get_longest_path(passcode: &'static str) -> String {
    let mut paths: HashSet<Path> = Default::default();
    let mut finished_paths: HashSet<Path> = Default::default();
    paths.insert(Path {
        passcode,
        current_position: Coord { x: 0, y: 0 },
        history: vec![],
    });
    while !paths.is_empty() {
        let mut new_paths = HashSet::new();
        for successor in paths.iter()
            .flat_map(|p| {
                get_successors_internal(p)
            }) {
            if successor.current_position == *END_POSITION {
                finished_paths.insert(successor);
            } else {
                new_paths.insert(successor);
            }
        }
        paths = new_paths;
    }

    finished_paths.into_iter().max_by(|a, b| a.history.len().cmp(&b.history.len()))
        .unwrap().history.into_iter().collect()
}

fn get_successors_internal(path: &Path) -> impl Iterator<Item=Path> + '_ {
    let current = &path.current_position;
    let is_open = |c: char| {
        if c.is_numeric() || c == 'a' {
            false
        } else {
            true
        }
    };
    let hash = path.get_hash();
    let mut chars = hash.chars();
    [
        (chars.next().unwrap(), current.y > 0, Direction::Up),
        (chars.next().unwrap(), current.y < END_POSITION.y, Direction::Down),
        (chars.next().unwrap(), current.x > 0, Direction::Left),
        (chars.next().unwrap(), current.x < END_POSITION.x, Direction::Right),
    ]
        .into_iter()
        .filter(move |(c, ok, _)| *ok && is_open(*c))
        .map(|(_, _, dir)| path.moved(dir))
}

fn get_successors(path: &Path) -> Vec<Successor<Path>> {
    get_successors_internal(path)
        .map(|p| Successor::new(p, 1))
        .collect()
}

fn distance_function(details: CurrentNodeDetails<Path>) -> i32 {
    details.current_node.current_position.get_manhattan_distance(&details.target_node.current_position) as i32
}

#[derive(Hash, Eq, Ord, PartialOrd, Debug, Clone)]
struct Path {
    passcode: &'static str,
    current_position: Coord,
    history: Vec<char>,
}

impl Path {
    pub(crate) fn get_hash(&self) -> String {
        let s: String = self.history.iter().fold(self.passcode.to_string(), |mut hash, next| {
            hash.push(*next);
            hash
        });
        let digest = md5::compute(s);
        format!("{:x}", digest)
    }
    pub(crate) fn moved(&self, direction: Direction) -> Self {
        let mut result = self.clone();
        match direction {
            Direction::Up => {
                result.current_position.y -= 1;
                result.history.push('U');
            }
            Direction::Down => {
                result.current_position.y += 1;
                result.history.push('D');
            }
            Direction::Left => {
                result.current_position.x -= 1;
                result.history.push('L');
            }
            Direction::Right => {
                result.current_position.x += 1;
                result.history.push('R');
            }
        }

        result
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        if self.current_position != other.current_position {
            return false;
        }
        // special case for end of maze
        if self.current_position == *END_POSITION {
            return true;
        }
        return self.history == other.history;
    }
}

impl AStarNode for Path {}

#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    pub(crate) fn get_manhattan_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}