use std::collections::{HashMap, HashSet};
use utils::a_star::*;

pub(crate) fn run() {
    let start = Coord { x: 1, y: 1 };
    // let end = Coord { x: 7, y: 4 };
    let end = Coord { x: 31, y: 39 };

    // let options = AStarOptions::print_stats();
    // let options = Some(&options);
    let options = None;
    let solution = a_star_search(start, &end, get_successors, distance_function, options).unwrap();

    Maze::print(&solution);
    println!("reaching {:?} would take a minimum of {} steps", end, solution.len() - 1);

    let mut visited: HashMap<Coord, usize> = Default::default();
    let start = Coord { x: 1, y: 1 };
    let steps = 50usize;
    visited.insert(start.clone(), steps);
    locations_can_be_visited_in_steps(&start, &mut visited, steps - 1);
    println!("{} locations can be visited in 50 steps", visited.len());
}

fn locations_can_be_visited_in_steps(from: &Coord, visited: &mut HashMap<Coord, usize>, steps_remaining: usize) {
    for successor in get_successors_internal(from).into_iter()
        .filter(|coord| !Maze::is_wall(coord)) {
        let entry = visited.entry(successor.clone()).or_default();
        if steps_remaining > *entry {
            *entry = steps_remaining;
            if steps_remaining > 0 {
                let steps_remaining = steps_remaining - 1;
                locations_can_be_visited_in_steps(&successor, visited, steps_remaining);
            }
        }
    }
}

fn get_successors_internal(current: &Coord) -> Vec<Coord> {
    (current.y.max(1) - 1..=current.y + 1)
        .map(move |y| Coord { x: current.x, y })
        .chain((current.x.max(1) - 1..=current.x + 1)
            .map(move |x| Coord { x, y: current.y }))
        .filter(|pos| {
            pos != current &&
                !Maze::is_wall(&pos)
        })
        .collect()
}

fn get_successors(current: &Coord) -> Vec<Successor<Coord>> {
    get_successors_internal(current)
        .into_iter()
        .map(|pos| Successor::new(pos, 1))
        .collect()
}

fn distance_function(details: CurrentNodeDetails<Coord>) -> i32 {
    details.current_node.manhattan_distance(&details.target_node) as i32
}

impl AStarNode for Coord {}

struct Maze {}

impl Maze {
    // const FAV_NUMBER: usize = 10;
    const FAV_NUMBER: usize = 1362;
    const WIDTH: usize = 32;
    const HEIGHT: usize = 40;

    fn is_wall(coord: &Coord) -> bool {
        let x = coord.x;
        let y = coord.y;
        let num = x * x + 3 * x + 2 * x * y + y + y * y + Self::FAV_NUMBER;
        let bits = get_count_set_bits(num);
        bits % 2 == 1
    }
    fn print(with_path: &[Coord]) {
        let path: HashSet<_> = with_path.into_iter().collect();
        for row in 0..Self::HEIGHT {
            println!("{}", (0..Self::WIDTH).map(|col| {
                let coord = Coord { x: col, y: row };
                if path.contains(&coord) {
                    'O'
                } else if Self::is_wall(&coord) {
                    '#'
                } else {
                    '.'
                }
            }).collect::<String>());
        }
    }
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    pub(crate) fn manhattan_distance(&self, other: &Self) -> usize {
        (self.x).abs_diff(other.x) +
            (self.y).abs_diff(other.y)
    }
}

fn get_count_set_bits(mut number: usize) -> usize {
    let n = (number as f64).log2().floor() as u32;
    let mut count = 0usize;
    for i in (0..=n).rev() {
        if number == 0 {
            break;
        }
        let val = 2usize.pow(i);
        if number >= val {
            number -= val;
            count += 1;
        }
    }
    count
}