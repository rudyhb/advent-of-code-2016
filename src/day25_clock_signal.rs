use crate::day12_leonardos_monorail::*;
use anyhow::anyhow;
use anyhow::Result;
use log::debug;
use std::str::FromStr;

pub(crate) fn run() {
    let _input = _get_input();
    let _input = _get_optimized_input();
    let instructions: Vec<ClockInstruction> =
        _input.lines().map(|line| line.parse().unwrap()).collect();

    // code divides (a + 2538) by 2. Remainder is the output.
    // smallest number should be ((1 * 2 + 1) * 2)... == a + 2538
    let a = get_min_a(2538);
    println!("using a = {}", a);

    let mut computer: Computer<ClockInstruction> = Computer::new();
    computer.set_register(Register::A, a);
    computer.run_while(instructions, &|state: &Computer<ClockInstruction>| {
        state.output.len() < 100
    });
    println!("output: {:?}", computer.output);
}

fn get_min_a(test_val: i32) -> i32 {
    let mut val = 1i32;
    let mut bit = 0;
    debug!("{}", val);
    while val < test_val {
        if bit == 0 {
            val *= 2;
            bit = 1;
        } else {
            val = val * 2 + 1;
            bit = 0;
        }
        debug!("{}", val);
    }
    val - test_val
}

#[derive(Debug)]
enum ClockInstruction {
    Copy(RegisterOrValue, Register),
    Increase(Register),
    Decrease(Register),
    JumpIfNotZero(RegisterOrValue, RegisterOrValue),
    Transmit(RegisterOrValue),
    Add(RegisterOrValue, Register),
    Multiply(RegisterOrValue, Register),
    Divide {
        value: RegisterOrValue,
        to: Register,
        remainder_to: Register,
    },
}

impl Instruction for ClockInstruction {
    fn run<TRetriever: Fn(&RegisterOrValue) -> i32>(
        &self,
        retrieve_fun: TRetriever,
    ) -> InstructionResult {
        match self {
            ClockInstruction::Copy(from, to) => {
                let val = retrieve_fun(from);
                let result = InstructionResult::update(to, Box::new(move |_| val));
                // if let RegisterOrValue::Register(Register::D) = from {
                //     if let Register::A = to {
                //         result = result._and_log_state();
                //     }
                // }
                // if let RegisterOrValue::Value(2) = from {
                //     if let Register::B = to {
                //         result = result._and_log_state();
                //     }
                // }

                result
            }
            ClockInstruction::Increase(r) => InstructionResult::update(r, Box::new(|val| val + 1)),
            ClockInstruction::Decrease(r) => InstructionResult::update(r, Box::new(|val| val - 1)),
            ClockInstruction::JumpIfNotZero(check, jump_val) => {
                if retrieve_fun(check) != 0 {
                    InstructionResult::jump(jump_val.clone())
                } else {
                    InstructionResult::do_nothing()
                }
            }
            ClockInstruction::Transmit(val) => InstructionResult::output(val.clone()),
            ClockInstruction::Add(val, to) => {
                let val = retrieve_fun(val);
                InstructionResult::update(to, Box::new(move |existing| existing + val))
            }
            ClockInstruction::Multiply(val, to) => {
                let val = retrieve_fun(val);
                InstructionResult::update(to, Box::new(move |existing| existing * val))
            }
            ClockInstruction::Divide {
                value,
                to,
                remainder_to,
            } => {
                let val = retrieve_fun(value);
                InstructionResult::update_pair(
                    to,
                    remainder_to,
                    Box::new(move |a, _| (a / val, a % 2)),
                )
            }
        }
    }

    fn toggle(&mut self) {
        panic!("invalid operation")
    }
}

impl FromStr for ClockInstruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut helper = InstructionParserHelper::new(s);
        match helper.next_word()? {
            "cpy" => Ok(Self::Copy(helper.next_rov()?, helper.next_register()?)),
            "inc" => Ok(Self::Increase(helper.next_register()?)),
            "dec" => Ok(Self::Decrease(helper.next_register()?)),
            "jnz" => Ok(Self::JumpIfNotZero(helper.next_rov()?, helper.next_rov()?)),
            "out" => Ok(Self::Transmit(helper.next_rov()?)),
            "mul" => Ok(Self::Multiply(helper.next_rov()?, helper.next_register()?)),
            "add" => Ok(Self::Add(helper.next_rov()?, helper.next_register()?)),
            "div" => Ok(Self::Divide {
                value: helper.next_rov()?,
                to: helper.next_register()?,
                remainder_to: helper.next_register()?,
            }),
            _ => Err(anyhow!("invalid command")),
        }
    }
}

fn _get_input() -> &'static str {
    "cpy a d
cpy 9 c
cpy 282 b
inc d
dec b
jnz b -2
dec c
jnz c -5
cpy d a
jnz 0 0
cpy a b
cpy 0 a
cpy 2 c
jnz b 2
jnz 1 6
dec b
dec c
jnz c -4
inc a
jnz 1 -7
cpy 2 b
jnz c 2
jnz 1 4
dec b
dec c
jnz 1 -4
jnz 0 0
out b
jnz a -19
jnz 1 -21"
}

fn _get_optimized_input() -> &'static str {
    "\
cpy 1 d
mul 9 d
mul 282 d
add a d
cpy d a
div 2 a b
out b
jnz a -2
jnz 1 -4"
}
