use rust_util::Day;
use std::{error::Error, fmt::Display, str::Lines};

pub struct Solve {
    registers: Registers,
    tape: Tape,
}

fn get_reg(c: char, lines: &mut Lines<'_>) -> Result<i64, Box<dyn Error>> {
    lines
        .next()
        .and_then(|s| s.strip_prefix(&format!("Register {}: ", c)))
        .and_then(|v| v.parse::<i64>().ok())
        .ok_or_else(|| "Register not found!".into())
}

impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut lines = value.lines();
        let a = get_reg('A', &mut lines)?;
        let b = get_reg('B', &mut lines)?;
        let c = get_reg('C', &mut lines)?;
        lines.next();
        let ops = lines
            .next()
            .and_then(|v| v.strip_prefix("Program: "))
            .map(|s| s.split(',').map(|n| n.parse::<u8>().unwrap()).collect())
            .unwrap();
        Ok(Solve {
            registers: Registers { a, b, c },
            tape: Tape { ops, pointer: 0 },
        })
    }
}

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let mut regs = self.registers.clone();
        let mut tape = self.tape.clone();
        Ok(Box::new(
            run(&mut tape, &mut regs)
                .iter()
                .map(|v| format!("{v}"))
                .collect::<Vec<_>>()
                .join(","),
        ))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(1))
    }
}

#[derive(Clone)]
struct Tape {
    ops: Vec<u8>,
    pointer: usize,
}

#[derive(Clone)]
struct Registers {
    a: i64,
    b: i64,
    c: i64,
}

enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Adv,
            1 => OpCode::Bxl,
            2 => OpCode::Bst,
            3 => OpCode::Jnz,
            4 => OpCode::Bxc,
            5 => OpCode::Out,
            6 => OpCode::Bdv,
            7 => OpCode::Cdv,
            _ => unreachable!("Reserved"),
        }
    }
}

impl Tape {
    fn take(&mut self) -> Option<u8> {
        let ret = self.ops.get(self.pointer).copied();
        self.pointer += 1;
        ret
    }

    fn jump_to(&mut self, operand: u8) {
        self.pointer = operand as usize;
    }
}

fn combo(operand: u8, regs: &Registers) -> i64 {
    match operand {
        0..=3 => operand.into(),
        4 => regs.a,
        5 => regs.b,
        6 => regs.c,
        _ => unreachable!("Reserved"),
    }
}

fn run(tape: &mut Tape, regs: &mut Registers) -> Vec<u8> {
    // Implicit: Tape is supposed to advance by 2 after an operand unless it's a jump.
    let mut out = Vec::new();
    (|| -> Option<()> {
        while let Some(opcode) = tape.take() {
            match OpCode::from(opcode) {
                OpCode::Adv => {
                    regs.a = regs.a / 2i64.pow(combo(tape.take()?, regs) as u32);
                }
                OpCode::Bdv => {
                    regs.b = regs.a / 2i64.pow(combo(tape.take()?, regs) as u32);
                }
                OpCode::Cdv => {
                    regs.c = regs.a / 2i64.pow(combo(tape.take()?, regs) as u32);
                }
                OpCode::Bxl => {
                    regs.b ^= tape.take()? as i64;
                }
                OpCode::Bxc => {
                    regs.b ^= regs.c;
                    tape.take()?; // Legacy
                }
                OpCode::Bst => {
                    regs.b = combo(tape.take()?, regs) % 8;
                }
                OpCode::Jnz => {
                    if regs.a != 0 {
                        let jmp = tape.take()?;
                        tape.jump_to(jmp);
                    }
                }
                OpCode::Out => {
                    out.push((combo(tape.take()?, regs) % 8) as u8);
                }
            }
        }
        None
    })();
    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn c1() {
        let mut regs = Registers { a: 0, b: 0, c: 9 };
        let mut tape = Tape {
            pointer: 0,
            ops: vec![2, 6],
        };
        run(&mut tape, &mut regs);
        assert_eq!(regs.b, 1)
    }

    #[test]
    fn c2() {
        assert_eq!(
            Solve {
                tape: Tape {
                    pointer: 0,
                    ops: vec![5, 0, 5, 1, 5, 4]
                },
                registers: Registers { a: 10, b: 0, c: 0 }
            }
            .p1()
            .map(|v| format!("{v}"))
            .unwrap(),
            "0,1,2"
        )
    }

    #[test]
    fn c3() {
        assert_eq!(
            Solve {
                tape: Tape {
                    pointer: 0,
                    ops: vec![0, 1, 5, 4, 3, 0]
                },
                registers: Registers {
                    a: 2024,
                    b: 0,
                    c: 0
                }
            }
            .p1()
            .map(|v| format!("{v}"))
            .unwrap(),
            "4,2,5,6,7,7,7,7,3,1,0"
        )
    }

    #[test]
    fn c4() {
        let mut regs = Registers { a: 0, b: 29, c: 0 };
        let mut tape = Tape {
            pointer: 0,
            ops: vec![1, 7],
        };
        run(&mut tape, &mut regs);
        assert_eq!(regs.b, 26)
    }

    #[test]
    fn c5() {
        let mut regs = Registers {
            a: 0,
            b: 2024,
            c: 43690,
        };
        let mut tape = Tape {
            pointer: 0,
            ops: vec![4, 0],
        };
        run(&mut tape, &mut regs);
        assert_eq!(regs.b, 44354)
    }

    #[test]
    fn full() {
        assert_eq!(
            Solve {
                tape: Tape {
                    pointer: 0,
                    ops: vec![0, 1, 5, 4, 3, 0]
                },
                registers: Registers { a: 729, b: 0, c: 9 }
            }
            .p1()
            .map(|v| format!("{v}"))
            .unwrap(),
            "4,6,3,5,6,3,5,2,1,0"
        )
    }
}
