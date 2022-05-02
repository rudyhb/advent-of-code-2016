use lazy_static::lazy_static;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

pub(crate) fn run() {
    let _input = ".^^.^.^^^^";
    let _input = _get_input();

    let _rows = 10usize;
    let _rows = 40usize;
    // let _rows = 400000usize;

    let mut room: Room = _input.parse().unwrap();
    room.fill_rows(_rows);

    println!("room:\n{:?}", room);
    println!("{} safe tiles", room.count_safe_tiles());
}

struct Room {
    rows: Vec<Vec<Tile>>,
}

impl Room {
    pub(crate) fn fill_rows(&mut self, to_num_rows: usize) {
        for i in 1..to_num_rows {
            let previous = &self.rows[i - 1];
            let mut row: Vec<Tile> = Vec::with_capacity(previous.len());
            for j in 0..previous.len() {
                let left = if j == 0 { &SAFE_TILE } else { &previous[j - 1] };
                let center = &previous[j];
                let right = if j == previous.len() - 1 {
                    &SAFE_TILE
                } else {
                    &previous[j + 1]
                };
                row.push(Tile::new(next_is_trap(left, center, right)));
            }

            self.rows.push(row);
        }
    }
    pub(crate) fn count_safe_tiles(&self) -> usize {
        self.rows
            .iter()
            .map(|r| r.iter().filter(|c| !c.is_trap).count())
            .sum()
    }
}

impl Debug for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            write!(
                f,
                "{}\n",
                row.iter()
                    .map(|c| if c.is_trap { '^' } else { '.' })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

impl FromStr for Room {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self {
            rows: vec![s
                .chars()
                .map(|c| Tile::new(if c == '^' { true } else { false }))
                .collect()],
        })
    }
}

fn next_is_trap(left: &Tile, center: &Tile, right: &Tile) -> bool {
    (left.is_trap && center.is_trap && !right.is_trap)
        || (center.is_trap && right.is_trap && !left.is_trap)
        || (left.is_trap && !center.is_trap && !right.is_trap)
        || (!left.is_trap && !center.is_trap && right.is_trap)
}

lazy_static! {
    static ref SAFE_TILE: Tile = Tile::new(false);
}

struct Tile {
    is_trap: bool,
}

impl Tile {
    pub(crate) fn new(is_trap: bool) -> Self {
        Self { is_trap }
    }
}

fn _get_input() -> &'static str {
    ".^^^^^.^^.^^^.^...^..^^.^.^..^^^^^^^^^^..^...^^.^..^^^^..^^^^...^.^.^^^^^^^^....^..^^^^^^.^^^.^^^.^^"
}
