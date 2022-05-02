use std::collections::HashMap;
use std::str::FromStr;

pub(crate) fn run() {
    let _input = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";
    let _input = _get_input();

    let instructions: Vec<Instruction> = _input.lines().map(|line| line.parse().unwrap()).collect();

    let mut computer = Computer::new();
    computer.set_register_c(1);
    computer.run(&instructions);
    println!("value at a: {}", computer.value_at_a());
}

struct Computer {
    registers: HashMap<Register, i32>,
}

impl Computer {
    pub(crate) fn new() -> Self {
        Self {
            registers: Default::default(),
        }
    }
    pub(crate) fn get_value(&self, rov: &RegisterOrValue) -> i32 {
        match rov {
            RegisterOrValue::Register(r) => self.registers.get(&r).map(|v| *v).unwrap_or_default(),
            RegisterOrValue::Value(v) => *v,
        }
    }
    pub(crate) fn update<TFun: Fn(i32) -> i32>(&mut self, r: &Register, func: TFun) {
        let val = self.registers.entry(*r).or_default();
        *val = func(*val);
    }
    pub(crate) fn run(&mut self, instructions: &[Instruction]) {
        let mut i = 0;
        while i < instructions.len() {
            match &instructions[i] {
                Instruction::Copy(from, to) => {
                    let val = self.get_value(from);
                    self.update(to, |_| val);
                }
                Instruction::Increase(r) => {
                    self.update(r, |val| val + 1);
                }
                Instruction::Decrease(r) => {
                    self.update(r, |val| val - 1);
                }
                Instruction::JumpIfNotZero(check, jump_val) => {
                    if self.get_value(check) != 0 {
                        i = (i as i32 + self.get_value(jump_val)) as usize;
                        continue;
                    }
                }
            }
            i += 1;
        }
    }
    pub(crate) fn value_at_a(&self) -> i32 {
        *self.registers.get(&Register::A).unwrap()
    }
    pub(crate) fn set_register_c(&mut self, value: i32) {
        self.registers.insert(Register::C, value);
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum Register {
    A,
    B,
    C,
    D,
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 {
            match s.chars().next().unwrap() {
                'a' => return Ok(Register::A),
                'b' => return Ok(Register::B),
                'c' => return Ok(Register::C),
                'd' => return Ok(Register::D),
                _ => {}
            }
        }
        Err(())
    }
}

enum RegisterOrValue {
    Register(Register),
    Value(i32),
}

impl FromStr for RegisterOrValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(register) = s.parse::<Register>() {
            Ok(Self::Register(register))
        } else {
            Ok(Self::Value(s.parse().or(Err(()))?))
        }
    }
}

enum Instruction {
    Copy(RegisterOrValue, Register),
    Increase(Register),
    Decrease(Register),
    JumpIfNotZero(RegisterOrValue, RegisterOrValue),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        match words.next().unwrap() {
            "cpy" => Ok(Self::Copy(
                words.next().unwrap().parse().unwrap(),
                words.next().unwrap().parse().unwrap(),
            )),
            "inc" => Ok(Self::Increase(words.next().unwrap().parse().unwrap())),
            "dec" => Ok(Self::Decrease(words.next().unwrap().parse().unwrap())),
            "jnz" => Ok(Self::JumpIfNotZero(
                words.next().unwrap().parse().unwrap(),
                words.next().unwrap().parse().unwrap(),
            )),
            _ => Err(()),
        }
    }
}

fn _get_input() -> &'static str {
    "cpy 1 a
cpy 1 b
cpy 26 d
jnz c 2
jnz 1 5
cpy 7 c
inc d
dec c
jnz c -2
cpy a c
inc a
dec b
jnz b -2
cpy c b
dec d
jnz d -6
cpy 14 c
cpy 14 d
inc a
dec d
jnz d -2
dec c
jnz c -5"
}
