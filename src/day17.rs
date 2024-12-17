use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Puter {
    a: usize,
    b: usize,
    c: usize,
    ip: usize,
    code: Vec<Instr>,
}

#[derive(Debug, Clone)]
enum Instr {
    /// The adv instruction (opcode 0) performs division. The numerator is the value in the A
    /// register. The denominator is found by raising 2 to the power of the instruction's combo
    /// operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by
    /// 2^B.) The result of the division operation is truncated to an integer and then written to
    /// the A register.
    ///
    Adv(CoOp),

    /// The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the
    /// instruction's literal operand, then stores the result in register B.
    Bxl(usize),

    /// The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby
    /// keeping only its lowest 3 bits), then writes that value to the B register.
    Bst(CoOp),

    /// The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A
    /// register is not zero, it jumps by setting the instruction pointer to the value of its
    /// literal operand; if this instruction jumps, the instruction pointer is not increased by 2
    /// after this instruction.
    Jnz(usize),

    /// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then
    /// stores the result in register B. (For legacy reasons, this instruction reads an operand but
    /// ignores it.)
    Bxc(usize),

    /// The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then
    /// outputs that value. (If a program outputs multiple values, they are separated by commas.)
    Out(CoOp),

    /// The bdv instruction (opcode 6) works exactly like the adv instruction except that the result
    /// is stored in the B register. (The numerator is still read from the A register.)
    Bdv(CoOp),

    /// The cdv instruction (opcode 7) works exactly like the adv instruction except that the result
    /// is stored in the C register. (The numerator is still read from the A register.)
    Cdv(CoOp),
}
#[derive(Debug, Clone, Copy)]
enum CoOp {
    Lit(usize),
    A,
    B,
    C,
}

impl Puter {
    fn resolve(&self, op: &CoOp) -> usize {
        match op {
            CoOp::Lit(l) => *l,
            CoOp::A => self.a,
            CoOp::B => self.b,
            CoOp::C => self.c,
        }
    }
}

#[aoc_generator(day17)]
fn parse(input: &str) -> Puter {
    let mut input = input.lines();
    let a = input.next().unwrap()[12..].parse().unwrap();
    let b = input.next().unwrap()[12..].parse().unwrap();
    let c = input.next().unwrap()[12..].parse().unwrap();
    input.next();
    let mut bytes = input.next().unwrap()[9..]
        .split(',')
        .map(|c| c.parse::<usize>().unwrap());
    let mut code = Vec::new();
    let mk = |v| match v {
        0..=3 => CoOp::Lit(v),
        4 => CoOp::A,
        5 => CoOp::B,
        6 => CoOp::C,
        _ => panic!(),
    };
    while let Some(instr) = bytes.next() {
        let op = bytes.next().unwrap();
        use Instr as I;
        code.push(match instr {
            0 => I::Adv(mk(op)),
            1 => I::Bxl(op),
            2 => I::Bst(mk(op)),
            3 => I::Jnz(op),
            4 => I::Bxc(op),
            5 => I::Out(mk(op)),
            6 => I::Bdv(mk(op)),
            7 => I::Cdv(mk(op)),
            _ => panic!(),
        });
    }

    Puter {
        a,
        b,
        c,
        ip: 0,
        code,
    }
}

#[aoc(day17, part1)]
fn part1(input: &Puter) -> String {
    let mut out = String::new();
    let mut puter = input.clone();

    loop {
        let Some(instr) = puter.code.get(puter.ip) else {
            break;
        };

        match instr {
            Instr::Adv(op) => {
                puter.a /= 2usize.pow(puter.resolve(op) as u32);
            }
            Instr::Bxl(op) => {
                puter.b ^= op;
            }
            Instr::Bst(op) => {
                puter.b = puter.resolve(op) % 8;
            }
            Instr::Jnz(op) => {
                if puter.a != 0 {
                    puter.ip = *op;

                    continue;
                }
            }
            Instr::Bxc(_) => {
                puter.b ^= puter.c;
            }
            Instr::Out(op) => {
                use std::fmt::Write;
                if !out.is_empty() {
                    out.push(',');
                }
                write!(out, "{}", puter.resolve(op) % 8).unwrap();
            }
            Instr::Bdv(op) => {
                puter.b = puter.a / 2usize.pow(puter.resolve(op) as u32);
            }
            Instr::Cdv(op) => {
                puter.c = puter.a / 2usize.pow(puter.resolve(op) as u32);
            }
        }
        puter.ip += 1;
    }

    out
}

#[aoc(day17, part2)]
fn part2(input: &Puter) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

        assert_eq!(part1(&parse(input)), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
