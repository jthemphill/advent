use std::collections::HashMap;

const ROOT: &str = "root";
const HUMN: &str = "humn";

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

fn evaluate(monkeys: &HashMap<&str, Monkey>, name: &str) -> Option<i64> {
    if name == HUMN {
        None
    } else {
        let monkey = monkeys.get(name).unwrap();
        match monkey.yell {
            Yell::Literal(n) => Some(n),
            Yell::Expr(op, m1, m2) => {
                if let Some(v1) = evaluate(monkeys, m1) {
                    if let Some(v2) = evaluate(monkeys, m2) {
                        Some(match op {
                            Op::Plus => v1 + v2,
                            Op::Minus => v1 - v2,
                            Op::Mul => v1 * v2,
                            Op::Div => v1 / v2,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }
}

fn make_equal(monkeys: &HashMap<&str, Monkey>, name: &str, target: i64) -> i64 {
    println!("Making {} evaluate to {}", name, target);
    let monkey = monkeys.get(name).unwrap();
    match monkey.yell {
        Yell::Literal(_) => {
            assert_eq!(name, HUMN);
            return target;
        }
        Yell::Expr(op, m1, m2) => match op {
            Op::Div => {
                if let Some(v1) = evaluate(monkeys, m1) {
                    make_equal(monkeys, m2, v1 / target)
                } else if let Some(v2) = evaluate(monkeys, m2) {
                    make_equal(monkeys, m1, target * v2)
                } else {
                    panic!("Neither branch evaluates??");
                }
            }
            Op::Minus => {
                if let Some(v1) = evaluate(monkeys, m1) {
                    make_equal(monkeys, m2, v1 - target)
                } else if let Some(v2) = evaluate(monkeys, m2) {
                    make_equal(monkeys, m1, target + v2)
                } else {
                    panic!("Neither branch evaluates??");
                }
            }
            Op::Plus => {
                if let Some(v1) = evaluate(monkeys, m1) {
                    make_equal(monkeys, m2, target - v1)
                } else if let Some(v2) = evaluate(monkeys, m2) {
                    make_equal(monkeys, m1, target - v2)
                } else {
                    panic!("Neither branch evaluates??");
                }
            }
            Op::Mul => {
                if let Some(v1) = evaluate(monkeys, m1) {
                    make_equal(monkeys, m2, target / v1)
                } else if let Some(v2) = evaluate(monkeys, m2) {
                    make_equal(monkeys, m1, target / v2)
                } else {
                    panic!("Neither branch evaluates??");
                }
            }
        },
    }
}

fn main() {
    let input: Vec<String> = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().to_owned())
        .collect();
    let monkeys: HashMap<&str, Monkey> = input
        .iter()
        .map(|line| {
            let mut split = line.split(": ");
            let name = split.next().unwrap();
            let expr = split.next().unwrap();
            let yell = if expr.contains(' ') {
                let mut toks = expr.split(' ');
                let m1 = toks.next().unwrap();
                let op_str = toks.next().unwrap();
                let m2 = toks.next().unwrap();
                let op = match op_str {
                    "+" => Op::Plus,
                    "-" => Op::Minus,
                    "/" => Op::Div,
                    "*" => Op::Mul,
                    _ => panic!("Expected +-/*, got {}", op_str),
                };
                Yell::Expr(op, m1, m2)
            } else {
                Yell::Literal(expr.parse().unwrap())
            };
            (name, Monkey { name, yell })
        })
        .collect();
    let root = monkeys.get(ROOT).unwrap();
    let humn_val = match root.yell {
        Yell::Expr(_, m1, m2) => {
            if let Some(v1) = evaluate(&monkeys, m1) {
                make_equal(&monkeys, m2, v1)
            } else if let Some(v2) = evaluate(&monkeys, m2) {
                make_equal(&monkeys, m1, v2)
            } else {
                panic!("humn appears in both branches")
            }
        }
        _ => panic!("Root is a literal?"),
    };
    println!("humn = {}", humn_val);
}
