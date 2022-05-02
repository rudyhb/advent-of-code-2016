use std::fmt::{Debug, Formatter};
use std::str::{FromStr, Split};

pub(crate) fn run() {
    let _input = "rect 3x2\nrotate column x=1 by 1\nrotate row y=0 by 4\nrotate column x=1 by 1";
    let _input = _get_input();

    let instructions: Vec<Instruction> = _input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
        .unwrap();

    let mut _lcd = Lcd::new(7, 3);
    let mut _lcd = Lcd::new(50, 6);
    let mut lcd = _lcd;

    for instruction in instructions.iter() {
        lcd.apply(instruction);
        println!("{:?}", lcd);
    }

    println!("LCD:\n{:?}", lcd);
    println!("lit count: {}", lcd.lit_count());
}

struct Lcd(Vec<Vec<Pixel>>);

impl Debug for Lcd {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            write!(
                f,
                "{}\n",
                row.iter()
                    .map(|p| if p.0 { '#' } else { '.' })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

impl Lcd {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self {
            0: vec![vec![Pixel(false); width]; height],
        }
    }
    pub(crate) fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Rect(width, height) => {
                for row in &mut self.0[0..*height] {
                    for pixel in &mut row[0..*width] {
                        pixel.0 = true;
                    }
                }
            }
            Instruction::RotateRow(row, amount) => {
                let reference = self.0[*row].clone();
                let len = reference.len();
                let amount = (((*amount as isize * -1) % len as isize) + len as isize) as usize;
                for (i, val) in self.0[*row].iter_mut().enumerate() {
                    *val = reference[(i + amount) % len];
                }
            }
            Instruction::RotateColumn(col, amount) => {
                let reference: Vec<_> = self.0.iter().map(|row| &row[*col]).copied().collect();
                let len = reference.len();
                let amount = (((*amount as isize * -1) % len as isize) + len as isize) as usize;
                for (i, row) in self.0.iter_mut().enumerate() {
                    let val = &mut row[*col];
                    *val = reference[(i + amount) % len];
                }
            }
        }
    }
    pub(crate) fn lit_count(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|p| p.0).count())
            .sum()
    }
}

#[derive(Clone, Copy)]
struct Pixel(bool);

enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_next =
            |parts: &mut Split<char>| parts.next().ok_or(())?.parse::<usize>().or(Err(()));
        let mut words = s.split_whitespace();
        Ok(match words.next().ok_or(())? {
            "rect" => {
                let mut parts = words.next().ok_or(())?.split('x');
                Self::Rect(parse_next(&mut parts)?, parse_next(&mut parts)?)
            }
            "rotate" => match words.next().ok_or(())? {
                "column" => {
                    let col = words.next().ok_or(())?[2..].parse::<usize>().or(Err(()))?;
                    words.next();
                    let val = words.next().ok_or(())?.parse::<usize>().or(Err(()))?;
                    Self::RotateColumn(col, val)
                }
                "row" => {
                    let row = words.next().ok_or(())?[2..].parse::<usize>().or(Err(()))?;
                    words.next();
                    let val = words.next().ok_or(())?.parse::<usize>().or(Err(()))?;
                    Self::RotateRow(row, val)
                }
                _ => return Err(()),
            },
            _ => return Err(()),
        })
    }
}

fn _get_input() -> &'static str {
    "rect 1x1
rotate row y=0 by 20
rect 1x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 3
rect 2x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 3
rect 2x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 4
rect 2x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 3
rect 2x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 5
rect 1x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 6
rect 5x1
rotate row y=0 by 2
rect 1x3
rotate row y=2 by 8
rotate row y=0 by 8
rotate column x=0 by 1
rect 7x1
rotate row y=2 by 24
rotate row y=0 by 20
rotate column x=5 by 1
rotate column x=4 by 2
rotate column x=2 by 2
rotate column x=0 by 1
rect 7x1
rotate column x=34 by 2
rotate column x=22 by 1
rotate column x=15 by 1
rotate row y=2 by 18
rotate row y=0 by 12
rotate column x=8 by 2
rotate column x=7 by 1
rotate column x=5 by 2
rotate column x=2 by 1
rotate column x=0 by 1
rect 9x1
rotate row y=3 by 28
rotate row y=1 by 28
rotate row y=0 by 20
rotate column x=18 by 1
rotate column x=15 by 1
rotate column x=14 by 1
rotate column x=13 by 1
rotate column x=12 by 2
rotate column x=10 by 3
rotate column x=8 by 1
rotate column x=7 by 2
rotate column x=6 by 1
rotate column x=5 by 1
rotate column x=3 by 1
rotate column x=2 by 2
rotate column x=0 by 1
rect 19x1
rotate column x=34 by 2
rotate column x=24 by 1
rotate column x=23 by 1
rotate column x=14 by 1
rotate column x=9 by 2
rotate column x=4 by 2
rotate row y=3 by 5
rotate row y=2 by 3
rotate row y=1 by 7
rotate row y=0 by 5
rotate column x=0 by 2
rect 3x2
rotate column x=16 by 2
rotate row y=3 by 27
rotate row y=2 by 5
rotate row y=0 by 20
rotate column x=8 by 2
rotate column x=7 by 1
rotate column x=5 by 1
rotate column x=3 by 3
rotate column x=2 by 1
rotate column x=1 by 2
rotate column x=0 by 1
rect 9x1
rotate row y=4 by 42
rotate row y=3 by 40
rotate row y=1 by 30
rotate row y=0 by 40
rotate column x=37 by 2
rotate column x=36 by 3
rotate column x=35 by 1
rotate column x=33 by 1
rotate column x=32 by 1
rotate column x=31 by 3
rotate column x=30 by 1
rotate column x=28 by 1
rotate column x=27 by 1
rotate column x=25 by 1
rotate column x=23 by 3
rotate column x=22 by 1
rotate column x=21 by 1
rotate column x=20 by 1
rotate column x=18 by 1
rotate column x=17 by 1
rotate column x=16 by 3
rotate column x=15 by 1
rotate column x=13 by 1
rotate column x=12 by 1
rotate column x=11 by 2
rotate column x=10 by 1
rotate column x=8 by 1
rotate column x=7 by 2
rotate column x=5 by 1
rotate column x=3 by 3
rotate column x=2 by 1
rotate column x=1 by 1
rotate column x=0 by 1
rect 39x1
rotate column x=44 by 2
rotate column x=42 by 2
rotate column x=35 by 5
rotate column x=34 by 2
rotate column x=32 by 2
rotate column x=29 by 2
rotate column x=25 by 5
rotate column x=24 by 2
rotate column x=19 by 2
rotate column x=15 by 4
rotate column x=14 by 2
rotate column x=12 by 3
rotate column x=9 by 2
rotate column x=5 by 5
rotate column x=4 by 2
rotate row y=5 by 5
rotate row y=4 by 38
rotate row y=3 by 10
rotate row y=2 by 46
rotate row y=1 by 10
rotate column x=48 by 4
rotate column x=47 by 3
rotate column x=46 by 3
rotate column x=45 by 1
rotate column x=43 by 1
rotate column x=37 by 5
rotate column x=36 by 5
rotate column x=35 by 4
rotate column x=33 by 1
rotate column x=32 by 5
rotate column x=31 by 5
rotate column x=28 by 5
rotate column x=27 by 5
rotate column x=26 by 3
rotate column x=25 by 4
rotate column x=23 by 1
rotate column x=17 by 5
rotate column x=16 by 5
rotate column x=13 by 1
rotate column x=12 by 5
rotate column x=11 by 5
rotate column x=3 by 1
rotate column x=0 by 1"
}
