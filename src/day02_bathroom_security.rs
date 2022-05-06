pub(crate) fn run() {
    let _input = "ULL
RRDDD
LURDL
UUUUD";
    let _input = _get_input();

    let use_fancy = true;

    let code = if use_fancy {
        obtain_code(&mut FancyKeypad::new(), _input)
    } else {
        obtain_code(&mut SimpleKeypad::new(), _input)
    };

    println!(
        "bathroom code for {} keypad is {}",
        if use_fancy { "fancy" } else { "simple" },
        code
    );
}

fn obtain_code<T>(keypad: &mut dyn Keypad<ValueType = T>, directions: &str) -> String {
    let directions: Vec<Vec<Direction>> = directions
        .split('\n')
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect();
    for directions in directions.iter() {
        keypad.go_to_next(directions);
    }
    keypad.get_code()
}

trait Keypad {
    type ValueType;
    fn get_code(&self) -> String;
    fn apply_direction(&mut self, direction: &Direction);
    fn get_value(&self) -> Self::ValueType;
    fn push_value(&mut self, value: Self::ValueType);
    fn go_to_next(&mut self, directions: &[Direction]) {
        for direction in directions {
            self.apply_direction(direction);
        }
        let value = self.get_value();
        self.push_value(value);
    }
}

struct FancyKeypad {
    pressed: Vec<char>,
    current: Coord,
}

impl Keypad for FancyKeypad {
    type ValueType = char;

    fn get_code(&self) -> String {
        self.pressed.iter().copied().collect()
    }
    fn apply_direction(&mut self, direction: &Direction) {
        let (target, other) = match direction {
            Direction::Up | Direction::Down => (&mut self.current.row, self.current.col),
            Direction::Left | Direction::Right => (&mut self.current.col, self.current.row),
        };
        let range = match other {
            0 | 4 => 2..=2,
            1 | 3 => 1..=3,
            2 => 0..=4,
            _ => panic!("current coordinate out of range"),
        };
        match direction {
            Direction::Down | Direction::Right => {
                if *target < *range.end() {
                    *target += 1;
                }
            }
            Direction::Up | Direction::Left => {
                if *target > *range.start() {
                    *target -= 1;
                }
            }
        }
        println!(
            "  >moved {:?} to {} ({:?})",
            direction,
            self.get_value(),
            self.current
        );
    }
    fn get_value(&self) -> char {
        match self.current.row {
            0 => '1',
            1 => char::from_digit(1 + self.current.col as u32, 10).unwrap(),
            2 => char::from_digit(5 + self.current.col as u32, 10).unwrap(),
            3 => ('A' as u8 + self.current.col as u8 - 1) as char,
            4 => 'D',
            _ => panic!("out of bounds get_value for fancy keypad"),
        }
    }
    fn push_value(&mut self, value: char) {
        self.pressed.push(value);
        println!("pressed {}", value);
    }
}

impl FancyKeypad {
    pub(crate) fn new() -> Self {
        Self {
            pressed: vec![],
            current: Coord { row: 2, col: 0 },
        }
    }
}

impl Keypad for SimpleKeypad {
    type ValueType = u8;

    fn get_code(&self) -> String {
        self.pressed
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &val)| 10u64.pow(i as u32) * val as u64)
            .sum::<u64>()
            .to_string()
    }
    fn apply_direction(&mut self, direction: &Direction) {
        let target = match direction {
            Direction::Up | Direction::Down => &mut self.current.row,
            Direction::Left | Direction::Right => &mut self.current.col,
        };
        match direction {
            Direction::Up | Direction::Right => {
                if *target < Self::WIDTH - 1 {
                    *target += 1;
                }
            }
            Direction::Down | Direction::Left => {
                if *target > 0 {
                    *target -= 1;
                }
            }
        }
        println!("  >moved {:?} to {}", direction, self.get_value());
    }
    #[inline]
    fn get_value(&self) -> u8 {
        let coord = &self.current;
        (1 + 3 * (2 - coord.row) + coord.col) as u8
    }
    fn push_value(&mut self, value: u8) {
        self.pressed.push(value);
        println!("pressed {}", value);
    }
}

struct SimpleKeypad {
    pressed: Vec<u8>,
    current: Coord,
}

impl SimpleKeypad {
    const WIDTH: usize = 3;

    pub(crate) fn new() -> Self {
        Self {
            pressed: vec![],
            current: Coord { row: 1, col: 1 },
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("out of bounds char"),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Coord {
    row: usize,
    col: usize,
}

fn _get_input() -> &'static str {
    "RRLLRLLRULLRUUUDRDLDDLLLDDDDDUUURRRRUUDLRULURRRDRUDRUUDDRUDLLLRLDDDUDRDDRRLLLLRLRLULUURDRURRUULDRRDUDURRUURURDLURULLDUDRDLUUUUDDURRLLLUDLDLRDRRRDULLDLDULLDRLDLDURDLRRULLDDLDRLLLUDDLLRDURULLDDDDDUURURLRLRRDUURUULRLLLULLRLULLUUDRRLLDURLDDDDULUUDLUDDDULRLDURDDRUUDRRUUURLLLULURUDRULDRDUDUDRRDDULRURLLRRLRRLLDLULURDRDRULDRDRURUDLLRRDUUULDDDUURDLULDLRLLURRURLLUDURDDRUDRDLLLLDLRLDLDDRDRRDUUULLUULRRDLURLDULLDLDUUUULLLDRURLRULLULRLULUURLLRDDRULDULRLDRRURLURUDLRRRLUDLDUULULLURLDDUDDLLUDRUDRLDUDURRRRLRUUURLUDDUDURDUDDDLLRLRDDURDRUUDUDRULURLRLDRULDRRLRLDDDRDDDRLDUDRLULDLUDLRLRRRLRDULDDLRRDDLDDULDLLDU
RULLUDDUDLULRRDLLDRUDLLLDURLLLURDURLRDRRDLRDRDLLURRULUULUDUDDLLRRULLURDRLDURDLDDUURLUURLDLDLRLDRLRUULDRLRLDRLRLUDULURDULLLDRUDULDURURRRUDURDUDLRDRRURULRRLRLRRRRRRDRUDLDRULDRUDLRDLRRUDULDLRLURRRLLDRULULRUDULRLULLRLULDRUDUULLRUULDULDUDDUUULLLDRDDRRDLURUUDRRLRRRDLRRLULLLLDLRUULDLLULURUURURDRURLLDUDRRURRURRUUDDRRDDRRRRUDULULRLUULRRDDRDDLLUDLDLULLRLDRLLUULDURLDRULDDUDRUUUURRLDDUDRUURUDLLDLDLURDLULDRLLLULLLUDLLDLD
RDLDULURDLULRRDLRLLLULRUULURULLLDLLDDRLLURUUUURDRLURLLRLRLLLULRDLURDURULULDDUDDUDRLRLDLULLURRRUULUDRDURRRUDDDLUDLDLRLRRLLLRUULLLLURRDDDRRRUURULRLDRRRLRLUDDRRULDDDRUUDDRLLDULRLUDUDLDLDDDUDDLLDDRDRDUDULDRRUDRDRRDRLUURDLRDDDULLDRRRRRUDRLURDUURRDDRLUDLURRRLRDDDLRRLUULRLURDUUURRDLDDULLLRURRRUDRLUDLLDDDDDUDDRDULLUUDDURRLULLUDULUUDRLDRRRLLURLRRLLDLLLLUDRUUUDDULLRDLLDUDUDUURRUUUDRUURDRDLLDLDDULLDDRRULDLDDUUURLDLULLLRRLLRDDULLDLDLDDLDLDULURRDURURDRDRRDLR
RDRLRRUUDRLDUDLLDLUDLUUDUDLRRUUDRDDDLDDLLLRRRUDULLRRRRRURRRLUDDDLRRRRUUULDURDRULLDLRURRUULUDRURRRRLRURLRDUUDUDUDRDDURRURUDLLLLLRURUULRUURLLURDRUURLUDDDRLDDURDLDUDRURDRLRRRRUURDDRRRRURDLUUDRLDRDUULURUDDULLURRDUDLUULLDURRURLUDUUDRDDDUUDDUUUULDLDUDDLUDUUDRURLLULRUUULLRRDDUDDLULDDUUUDLUDDLDDLLRUUDRULLRRDRLLDLLRRLULLRRDDRLRDUULLLUULLDLLUDUDDLRDULUDLDLUDDRRRRDUDLUULLULDLRRDLULRLRRRULRURRDRLULDDUDLDLDULLURLLRDLURRULURDLURLUDRDRRUUDRLLUDDRLRDDUURLRRDUDLDRURDUUUDRRLLRDLDLLDRRURLUDURUULDUDLDDDDRUULLDDRLRURRDURLURRLDDRRRRLRLRDRURUDDRDLDRURLULDDL
RULRDLDDLRURDDDDDDRURLLLDDDUUULLRRDLDLURUURLUDLURRLUDUURDULDRUULDDURULDUULDDULLLUDLRULDRLDLRDDRRDLDDLLDRRUDDUDRDUULUDLLLDDLUUULDDUUULRRDULLURLULDLRLLLRLURLLRLRLDRDURRDUUDDURRULDDURRULRDRDUDLRRDRLDULULDRDURDURLLLDRDRLULRDUURRUUDURRDRLUDDRRLDLDLULRLLRRUUUDDULURRDRLLDLRRLDRLLLLRRDRRDDLDUULRLRRULURLDRLRDULUDRDLRUUDDDURUDLRLDRRUDURDDLLLUDLRLURDUDUDULRURRDLLURLLRRRUDLRRRLUDURDDDDRRDLDDLLDLRDRDDRLLLURDDRDRLRULDDRRLUURDURDLLDRRRDDURUDLDRRDRUUDDDLUDULRUUUUDRLDDD"
}
