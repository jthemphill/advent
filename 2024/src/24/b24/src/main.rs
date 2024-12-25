use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

use regex::Regex;

type Registers<'s> = HashMap<&'s str, i64>;
type AssignmentBinop<'s> = (&'s str, Op, &'s str);
type Assignments<'s> = HashMap<&'s str, AssignmentBinop<'s>>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn color(&self) -> &'static str {
        match *self {
            Op::And => "red",
            Op::Or => "blue",
            Op::Xor => "green",
        }
    }
}

fn make_swaps<'s>(pairs: Vec<(&'s str, &'s str)>) -> HashMap<&'s str, &'s str> {
    let mut swaps = HashMap::new();
    for (a, b) in pairs {
        swaps.insert(a, b);
        swaps.insert(b, a);
    }
    swaps
}

fn to_viz(assignments: &Assignments, swaps: &HashMap<&str, &str>) {
    assert_eq!(swaps.len() % 2, 0);

    println!("digraph {{");
    println!("  rankdir = TB;");

    println!("  {{ rank=source;");
    for i in 0..45 {
        println!("  x{i:02}; y{i:02}");
    }
    println!("  }}");

    println!("  {{ rank=sink;");
    for i in 0..=45 {
        println!("  z{i:02};");
    }
    println!("  }}");

    for (result, (lhs, op, rhs)) in assignments {
        let result = swaps.get(result).unwrap_or(result);
        println!("  {{ {lhs} {rhs} }} -> {result} [color={}];", op.color());
    }

    println!("}}");
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let init_re = Regex::new(r"(.*): ([01])").unwrap();
    let assign_re = Regex::new(r"(.*) (.*) (.*) -> (.*)").unwrap();

    let mut registers: Registers = HashMap::new();
    let mut assignments: Assignments = HashMap::new();
    let mut nodes: HashSet<&str> = HashSet::new();
    for line in input.split('\n') {
        if let Some(init) = init_re.captures(line) {
            let label = init.get(1).unwrap().as_str();
            nodes.insert(label);
            registers.insert(label, init.get(2).unwrap().as_str().parse().unwrap());
        } else if let Some(assign) = assign_re.captures(line) {
            let lhs = assign.get(1).unwrap().as_str();
            let binop = match assign.get(2).unwrap().as_str() {
                "AND" => Op::And,
                "OR" => Op::Or,
                "XOR" => Op::Xor,
                s => panic!("Unexpected binop: {s}"),
            };
            let rhs = assign.get(3).unwrap().as_str();
            let result = assign.get(4).unwrap().as_str();
            assignments.insert(result, (lhs, binop, rhs));

            nodes.insert(lhs);
            nodes.insert(rhs);
            nodes.insert(result);
        }
    }

    let swaps = make_swaps(vec![]);
    to_viz(&assignments, &swaps);
}
