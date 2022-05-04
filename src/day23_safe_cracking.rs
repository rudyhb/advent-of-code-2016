use crate::day12_leonardos_monorail::*;
use anyhow::anyhow;
use anyhow::Result;
use log::debug;
use std::str::FromStr;

pub(crate) fn run() {
    let _input = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";
    let _input = _get_input();

    let instructions: Vec<SafeInstruction> =
        _input.lines().map(|line| line.parse().unwrap()).collect();

    let mut computer: Computer<SafeInstruction> = Computer::new();
    computer.set_register(Register::A, 12);
    computer.run(instructions);
    println!("value at a: {}", computer.value_at(&Register::A));
}

#[derive(Debug, Clone)]
enum SafeInstruction {
    Copy(RegisterOrValue, RegisterOrValue),
    Increase(RegisterOrValue),
    Decrease(RegisterOrValue),
    JumpIfNotZero(RegisterOrValue, RegisterOrValue),
    Toggle(RegisterOrValue),
}

impl Instruction for SafeInstruction {
    fn run<TRetriever: Fn(&RegisterOrValue) -> i32>(
        &self,
        retrieve_fun: TRetriever,
    ) -> InstructionResult {
        match self {
            SafeInstruction::Copy(from, to) => {
                match to {
                    RegisterOrValue::Register(to) => {
                        let val = retrieve_fun(from);
                        InstructionResult::update(to, Box::new(move |_| val))
                    }
                    RegisterOrValue::Value(_) => {
                        // skip invalid instruction
                        InstructionResult::do_nothing()
                    }
                }
            }
            SafeInstruction::Increase(r) => {
                match r {
                    RegisterOrValue::Register(r) => {
                        InstructionResult::update(r, Box::new(|val| val + 1))
                    }
                    RegisterOrValue::Value(_) => {
                        // skip invalid instruction
                        InstructionResult::do_nothing()
                    }
                }
            }
            SafeInstruction::Decrease(r) => {
                match r {
                    RegisterOrValue::Register(r) => {
                        InstructionResult::update(r, Box::new(|val| val - 1))
                    }
                    RegisterOrValue::Value(_) => {
                        // skip invalid instruction
                        InstructionResult::do_nothing()
                    }
                }
            }
            SafeInstruction::JumpIfNotZero(check, jump_val) => {
                if retrieve_fun(check) != 0 {
                    InstructionResult::jump(jump_val.clone())
                } else {
                    InstructionResult::do_nothing()
                }
            }
            SafeInstruction::Toggle(val) => InstructionResult::toggle(val.clone()),
        }
    }

    fn toggle(&mut self) {
        let toggle_to = match self {
            SafeInstruction::Copy(a, b) => SafeInstruction::JumpIfNotZero(a.clone(), b.clone()),
            SafeInstruction::Increase(a) => SafeInstruction::Decrease(a.clone()),
            SafeInstruction::Decrease(a) => SafeInstruction::Increase(a.clone()),
            SafeInstruction::JumpIfNotZero(a, b) => SafeInstruction::Copy(a.clone(), b.clone()),
            SafeInstruction::Toggle(a) => SafeInstruction::Increase(a.clone()),
        };
        debug!("instruction {:?} changed to {:?}", self, toggle_to);
        *self = toggle_to;
    }
}

impl FromStr for SafeInstruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut helper = InstructionParserHelper::new(s);
        match helper.next_word()? {
            "cpy" => Ok(Self::Copy(helper.next_rov()?, helper.next_rov()?)),
            "inc" => Ok(Self::Increase(helper.next_rov()?)),
            "dec" => Ok(Self::Decrease(helper.next_rov()?)),
            "jnz" => Ok(Self::JumpIfNotZero(helper.next_rov()?, helper.next_rov()?)),
            "tgl" => Ok(Self::Toggle(helper.next_rov()?)),
            _ => Err(anyhow!("invalid command")),
        }
    }
}

fn _get_input() -> &'static str {
    "cpy a b
dec b
cpy a d
cpy 0 a
cpy b c
inc a
dec c
jnz c -2
dec d
jnz d -5
dec b
cpy b c
cpy c d
dec d
inc c
jnz d -2
tgl c
cpy -16 c
jnz 1 c
cpy 98 c
jnz 86 d
inc a
inc d
jnz d -2
inc c
jnz c -5"
}
