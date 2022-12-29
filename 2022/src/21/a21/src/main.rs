use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Op {
    Plus,
    Minus,
    Div,
    Mul,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Yell<'a> {
    Literal(i64),
    Expr(Op, &'a str, &'a str),
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Monkey<'a> {
    name: &'a str,
    yell: Yell<'a>,
}

fn toposort<'a>(monkeys: &Vec<Monkey<'a>>) -> Vec<Monkey<'a>> {
    let mut sorted_monkeys = vec![];
    let mut finished = HashSet::new();
    while sorted_monkeys.len() < monkeys.len() {
        for m in monkeys {
            if !finished.contains(m.name) {
                match m.yell {
                    Yell::Literal(_) => {
                        sorted_monkeys.push(*m);
                        finished.insert(m.name);
                    }
                    Yell::Expr(_, m1, m2) => {
                        if finished.contains(m1) && finished.contains(m2) {
                            sorted_monkeys.push(*m);
                            finished.insert(m.name);
                        }
                    }
                }
            }
        }
    }
    sorted_monkeys
}

fn main() {
    let input: Vec<String> = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().to_owned())
        .collect();
    let monkeys: Vec<Monkey> = input
        .iter()
        .map(|line| {
            let mut split = line.split(": ");
            let name = split.next().unwrap();
            let expr = split.next().unwrap();
            let yell = if expr.contains(' ') {
                let mut toks = expr.split(' ');
                let m1 = toks.next().unwrap();
                let op_str = toks.next().unwrap();
                let op = match op_str {
                    "+" => Op::Plus,
                    "-" => Op::Minus,
                    "/" => Op::Div,
                    "*" => Op::Mul,
                    _ => panic!("Expected +-/*, got {}", op_str),
                };
                let m2 = toks.next().unwrap();
                Yell::Expr(op, m1, m2)
            } else {
                Yell::Literal(expr.parse().unwrap())
            };
            Monkey { name, yell }
        })
        .collect();
    let sorted_monkeys = toposort(&monkeys);
    let mut values = HashMap::new();
    for m in sorted_monkeys {
        values.insert(
            m.name,
            match m.yell {
                Yell::Literal(n) => n,
                Yell::Expr(op, m1, m2) => {
                    let v1 = *values.get(m1).unwrap();
                    let v2 = *values.get(m2).unwrap();
                    match op {
                        Op::Plus => v1 + v2,
                        Op::Minus => v1 - v2,
                        Op::Mul => v1 * v2,
                        Op::Div => v1 / v2,
                    }
                }
            },
        );
    }
    println!("Root: {}", values.get("root").unwrap());
}
