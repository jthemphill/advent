use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

use regex::Regex;

type Registers<'s> = HashMap<&'s str, i64>;
type AssignmentBinop<'s> = (&'s str, Op, &'s str);
type Assignments<'s> = HashMap<&'s str, AssignmentBinop<'s>>;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Op {
    And,
    Or,
    Xor,
}

fn toposort<'s>(assigns: &'s Assignments) -> Vec<&'s str> {
    let mut new_assigns = vec![];
    let mut to_visit: HashSet<&str> = assigns.keys().cloned().collect();

    fn visit<'s>(
        to_visit: &mut HashSet<&'s str>,
        assigns: &Assignments<'s>,
        new_assigns: &mut Vec<&'s str>,
        node: &'s str,
    ) {
        if !to_visit.remove(node) {
            return;
        }
        let (lhs, _, rhs) = assigns.get(node).unwrap();
        visit(to_visit, assigns, new_assigns, lhs);
        visit(to_visit, assigns, new_assigns, rhs);
        new_assigns.push(node);
    }

    while !to_visit.is_empty() {
        let node: &str = to_visit.iter().next().unwrap();
        visit(&mut to_visit, assigns, &mut new_assigns, node);
    }
    new_assigns
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let init_re = Regex::new(r"(.*): ([01])").unwrap();
    let assign_re = Regex::new(r"(.*) (.*) (.*) -> (.*)").unwrap();

    let mut registers: Registers = HashMap::new();
    let mut assigns: Assignments = HashMap::new();
    let mut z_labels = vec![];
    for line in input.split('\n') {
        if let Some(init) = init_re.captures(line) {
            registers.insert(
                init.get(1).unwrap().as_str(),
                init.get(2).unwrap().as_str().parse().unwrap(),
            );
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
            assigns.insert(result, (lhs, binop, rhs));

            if result.starts_with('z') {
                z_labels.push(result);
            }
        }
    }

    let sorted_results = toposort(&assigns);
    for result in sorted_results {
        let (lhs, op, rhs) = assigns.get(result).unwrap();
        let &lhs_value = registers.get(*lhs).unwrap();
        let &rhs_value = registers.get(*rhs).unwrap();
        let result_value = match op {
            Op::And => lhs_value & rhs_value,
            Op::Or => lhs_value | rhs_value,
            Op::Xor => lhs_value ^ rhs_value,
        };
        registers.insert(result, result_value);
    }

    let mut z: i64 = 0;
    for z_label in z_labels {
        let mut z_bit = 0;
        for &digit in z_label.as_bytes()[1..].iter() {
            z_bit *= 10;
            z_bit += (digit - b'0') as i64;
        }

        let z_value = registers.get(z_label).unwrap();
        z |= z_value << z_bit;
    }
    println!("{z}");
}
