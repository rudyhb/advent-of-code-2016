use anyhow::{anyhow, Result};
use log::debug;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::str::{FromStr, SplitWhitespace};

pub(crate) fn run() {
    let _input = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";
    let _input = _get_input();

    let instructions: Vec<LeonardoInstruction> =
        _input.lines().map(|line| line.parse().unwrap()).collect();

    let mut computer: Computer<LeonardoInstruction> = Computer::new();
    computer.set_register(Register::C, 1);
    computer.run(instructions);
    println!("value at a: {}", computer.value_at(&Register::A));
}

pub(crate) struct Computer<TInstruction: Instruction> {
    registers: HashMap<Register, i32>,
    phantom_parameter: PhantomData<TInstruction>,
}

impl<TInstruction: Instruction> Computer<TInstruction> {
    pub(crate) fn new() -> Self {
        Self {
            registers: Default::default(),
            phantom_parameter: PhantomData,
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
    pub(crate) fn run(&mut self, mut instructions: Vec<TInstruction>) {
        let mut i = 0;
        let mut ran = 0usize;
        while i < instructions.len() {
            let result =
                instructions[i].run(|rov: &RegisterOrValue| -> i32 { self.get_value(rov) });
            if let Some((register, update)) = result.update_register {
                self.update(register, update);
            }
            let next_instruction = if let Some(jump) = result.jump {
                (i as i32 + self.get_value(&jump)) as usize
            } else {
                i + 1
            };
            if let Some(toggle) = result.toggle {
                let j = (i as i32 + self.get_value(&toggle)) as usize;
                if j < instructions.len() {
                    instructions[j].toggle();
                }
            }

            i = next_instruction;
            ran += 1;
            if ran % 1_000_000 == 0 {
                debug!("ran {} total instructions, now at {}, a={}", ran, i, self.value_at(&Register::A));
            }
        }
    }
    pub(crate) fn value_at(&self, register: &Register) -> i32 {
        *self.registers.get(register).unwrap()
    }
    pub(crate) fn set_register(&mut self, register: Register, value: i32) {
        self.registers.insert(register, value);
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub(crate) enum Register {
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

#[derive(Clone)]
pub(crate) enum RegisterOrValue {
    Register(Register),
    Value(i32),
}

impl Debug for RegisterOrValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RegisterOrValue::Register(r) => {
                    match r {
                        Register::A => "A".to_string(),
                        Register::B => "B".to_string(),
                        Register::C => "C".to_string(),
                        Register::D => "D".to_string(),
                    }
                }
                RegisterOrValue::Value(v) => {
                    v.to_string()
                }
            }
        )
    }
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

pub(crate) trait Instruction {
    fn run<TRetriever: Fn(&RegisterOrValue) -> i32>(
        &self,
        retrieve_fun: TRetriever,
    ) -> InstructionResult;
    fn toggle(&mut self);
}

pub(crate) struct InstructionResult<'a> {
    pub(crate) update_register: Option<(&'a Register, Box<dyn Fn(i32) -> i32>)>,
    pub(crate) jump: Option<RegisterOrValue>,
    pub(crate) toggle: Option<RegisterOrValue>,
}

impl<'a> InstructionResult<'a> {
    pub(crate) fn update(register: &'a Register, func: Box<dyn Fn(i32) -> i32>) -> Self {
        Self {
            update_register: Some((register, func)),
            jump: None,
            toggle: None,
        }
    }
    pub(crate) fn jump(rov: RegisterOrValue) -> Self {
        Self {
            update_register: None,
            jump: Some(rov),
            toggle: None,
        }
    }
    pub(crate) fn toggle(rov: RegisterOrValue) -> Self {
        Self {
            update_register: None,
            jump: None,
            toggle: Some(rov),
        }
    }
    pub(crate) fn do_nothing() -> Self {
        Self {
            update_register: None,
            jump: None,
            toggle: None,
        }
    }
}

enum LeonardoInstruction {
    Copy(RegisterOrValue, Register),
    Increase(Register),
    Decrease(Register),
    JumpIfNotZero(RegisterOrValue, RegisterOrValue),
}

impl Instruction for LeonardoInstruction {
    fn run<TRetriever: Fn(&RegisterOrValue) -> i32>(
        &self,
        retrieve_fun: TRetriever,
    ) -> InstructionResult {
        match self {
            LeonardoInstruction::Copy(from, to) => {
                let val = retrieve_fun(from);
                InstructionResult::update(to, Box::new(move |_| val))
            }
            LeonardoInstruction::Increase(r) => {
                InstructionResult::update(r, Box::new(|val| val + 1))
            }
            LeonardoInstruction::Decrease(r) => {
                InstructionResult::update(r, Box::new(|val| val - 1))
            }
            LeonardoInstruction::JumpIfNotZero(check, jump_val) => {
                if retrieve_fun(check) != 0 {
                    InstructionResult::jump(jump_val.clone())
                } else {
                    InstructionResult::do_nothing()
                }
            }
        }
    }

    fn toggle(&mut self) {
        panic!("invalid operation");
    }
}

pub(crate) struct InstructionParserHelper<'a>(SplitWhitespace<'a>);

impl<'a> InstructionParserHelper<'a> {
    pub(crate) fn new(s: &'a str) -> Self {
        Self(s.split_whitespace())
    }
    pub(crate) fn next_rov(&mut self) -> Result<RegisterOrValue> {
        self.0
            .next()
            .ok_or(anyhow!("invalid input"))?
            .parse::<RegisterOrValue>()
            .or(Err(anyhow!("parse error")))
    }
    pub(crate) fn next_register(&mut self) -> Result<Register> {
        self.0
            .next()
            .ok_or(anyhow!("invalid input"))?
            .parse::<Register>()
            .or(Err(anyhow!("parse error")))
    }
    pub(crate) fn next_word(&mut self) -> Result<&'a str> {
        self.0.next().ok_or(anyhow!("input too short"))
    }
}

impl FromStr for LeonardoInstruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut helper = InstructionParserHelper::new(s);
        match helper.next_word()? {
            "cpy" => Ok(Self::Copy(helper.next_rov()?, helper.next_register()?)),
            "inc" => Ok(Self::Increase(helper.next_register()?)),
            "dec" => Ok(Self::Decrease(helper.next_register()?)),
            "jnz" => Ok(Self::JumpIfNotZero(helper.next_rov()?, helper.next_rov()?)),
            _ => Err(anyhow!("invalid command")),
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
