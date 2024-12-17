use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Puter {
    a: usize,
    b: usize,
    c: usize,
    ip: usize,
    code: Vec<Instr>,
    raw: String,
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

impl std::fmt::Display for CoOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoOp::Lit(l) => write!(f, "{l}"),
            CoOp::A => write!(f, "A"),
            CoOp::B => write!(f, "B"),
            CoOp::C => write!(f, "C"),
        }
    }
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

fn eval(puter: &mut Puter, out: &mut String, sp: bool) {
    loop {
        if sp && !puter.raw.starts_with(out.as_str()) {
            break;
        }

        let Some(instr) = puter.code.get(puter.ip) else {
            break;
        };
        //        println!("{instr:?}");
        //        println!("{:?}, {:?}", out, puter.raw);
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
}

#[aoc_generator(day17)]
fn parse(input: &str) -> Puter {
    let mut input = input.lines();
    let a = input.next().unwrap()[12..].parse().unwrap();
    let b = input.next().unwrap()[12..].parse().unwrap();
    let c = input.next().unwrap()[12..].parse().unwrap();
    input.next();
    let raw = input.next().unwrap()[9..].to_owned();
    let mut bytes = raw.split(',').map(|c| c.parse::<usize>().unwrap());
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
        raw,
    }
}

#[aoc(day17, part1)]
fn part1(input: &Puter) -> String {
    let mut out = String::new();
    let mut puter = input.clone();
    eval(&mut puter, &mut out, false);
    out
}

#[aoc(day17, part2)]
fn part2(input: &Puter) -> usize {
    for i in &input.code {
        match i {
            Instr::Adv(c) => println!("a /= {c}"),
            Instr::Bxl(l) => println!("b ^= {l}"),
            Instr::Bst(c) => println!("b = {c} % 8"),
            Instr::Jnz(l) => println!("jump {l}"),
            Instr::Bxc(l) => println!("b ^= c"),
            Instr::Out(c) => println!("write {c} % 8"),
            Instr::Bdv(c) => println!("b = a / {c}"),
            Instr::Cdv(c) => println!("c = a / {c}"),
        }
    }
    println!("{:?}", input.code);
    println!("{:?}", input.raw.len());
    let mut out = String::new();

    for s in 0..1000000000 {
        for p3 in &[0b0001010100, 0b1001010100] {
            for p2 in &[0b1000100101, 0b1011000000, 0b1011000100, 0b1011001000] {
                for p1 in &[0b1110011011, 0b1110011101] {
                    let a = p1 + p2 * 2usize.pow(10) + p3 * 2usize.pow(20) + s * 2usize.pow(30);
                    let mut puter = input.clone();
                    out.clear();
                    puter.a = a;
                    eval(&mut puter, &mut out, true);

                    //                    println!("{:#064b} => {}", a, out.len(),);

                    if out == puter.raw {
                        return a;
                    }
                }
            }
        }
    }
    panic!("Can't find a solution");
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
        let input = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!(part2(&parse(input)), 117440);
    }
}
