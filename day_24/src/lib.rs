use std::cell::RefCell;
use std::collections::VecDeque;
use std::{convert::Infallible, str::FromStr};

#[derive(Default, Debug)]
struct State {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
enum Operand {
    Register(String),
    Literal(i64),
}

#[derive(Debug)]
enum Operation {
    Inp(Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Mod(Operand, Operand),
    Eql(Operand, Operand),
}

#[derive(Default, Debug)]
struct Alu {
    state: RefCell<State>,
    ops: Vec<Operation>,
}

impl Alu {
    fn new() -> Self {
        Self::default()
    }

    fn compute(&self, input: VecDeque<u8>) {
        let mut input = VecDeque::from(input);
        // println!("{:?}", input);

        for op in &self.ops {
            match op {
                Operation::Inp(reg) => {
                    let value = input.pop_front().unwrap() as i64;
                    self.set_value(reg, value);
                }
                Operation::Add(a, b) => self.set_value(a, self.get_value(a) + self.get_value(b)),
                Operation::Mul(a, b) => self.set_value(a, self.get_value(a) * self.get_value(b)),
                Operation::Div(a, b) => self.set_value(a, self.get_value(a) / self.get_value(b)),
                Operation::Mod(a, b) => self.set_value(a, self.get_value(a) % self.get_value(b)),
                Operation::Eql(a, b) => self.set_value(
                    a,
                    if self.get_value(a) == self.get_value(b) {
                        1
                    } else {
                        0
                    },
                ),
            }
        }
    }

    fn get_value(&self, operand: &Operand) -> i64 {
        // println!("GET: {:?}", operand);
        match operand {
            Operand::Register(x) => match x.as_str() {
                "w" => self.state.borrow().w,
                "x" => self.state.borrow().x,
                "y" => self.state.borrow().y,
                "z" => self.state.borrow().z,
                _ => unreachable!(),
            },
            Operand::Literal(x) => *x,
        }
    }

    fn set_value(&self, operand: &Operand, value: i64) {
        // println!("SET: {} -> {:?}", value, operand);
        match operand {
            Operand::Register(x) => match x.as_str() {
                "w" => self.state.borrow_mut().w = value,
                "x" => self.state.borrow_mut().x = value,
                "y" => self.state.borrow_mut().y = value,
                "z" => self.state.borrow_mut().z = value,
                _ => unreachable!(),
            },
            Operand::Literal(_) => unreachable!(),
        }
    }

    fn reset(&self) {
        *self.state.borrow_mut() = State::default();
    }
}

impl FromStr for Operand {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "w" | "x" | "y" | "z" => Operand::Register(s.to_string()),
            _ => Operand::Literal(s.parse().unwrap()),
        })
    }
}

fn parse_input(input: Option<&str>) -> Alu {
    input
        .unwrap_or(include_str!("../input.txt"))
        .lines()
        .fold(Alu::new(), |mut alu, line| {
            let (op, args) = line.split_once(" ").unwrap();
            let op = match op {
                "inp" => Operation::Inp(args.parse().unwrap()),
                _ => {
                    let (a, b) = args.split_once(" ").unwrap();
                    let (a, b) = (a.parse().unwrap(), b.parse().unwrap());
                    match op {
                        "add" => Operation::Add(a, b),
                        "mul" => Operation::Mul(a, b),
                        "div" => Operation::Div(a, b),
                        "mod" => Operation::Mod(a, b),
                        "eql" => Operation::Eql(a, b),
                        _ => unreachable!(),
                    }
                }
            };
            alu.ops.push(op);
            alu
        })
}

fn number_to_vec(n: i64) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push((n % 10) as u8);
        n = n / 10;
    }
    digits.push(n as u8);
    digits.reverse();
    digits
}

pub fn part_a(input: Option<&str>) -> u64 {
    let alu = parse_input(input);
    // for i in (11111111111111..=99999999999999).rev() {
    //     alu.compute(VecDeque::from(number_to_vec(i)));
    //     if alu.state.borrow().z == 0 {
    //         return i as u64;
    //     }
    //     alu.reset();
    // }
    unreachable!()
}

pub fn part_b(input: Option<&str>) -> u64 {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::Operand;
    use std::collections::VecDeque;

    #[test]
    fn test_part_a_works() {
        let alu = super::parse_input(Some(include_str!("../example.txt")));
        alu.compute(VecDeque::from([6]));
        assert_eq!(alu.get_value(&Operand::Register("w".to_string())), 0);
        assert_eq!(alu.get_value(&Operand::Register("x".to_string())), 1);
        assert_eq!(alu.get_value(&Operand::Register("y".to_string())), 1);
        assert_eq!(alu.get_value(&Operand::Register("z".to_string())), 0);
    }

    #[test]
    fn test_part_a() {
        assert_eq!(super::part_a(None), 123);
    }
}
