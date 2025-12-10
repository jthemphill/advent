use std::{fmt::Display, io::BufRead};

use nalgebra::*;

#[derive(Clone, Debug)]
struct Machine {
    joltage_goal: MatrixXx1<f64>,
    buttons: DMatrix<f64>,
}

impl Machine {
    fn from_str(line: &str) -> Self {
        let mut joltage_goal = vec![];
        let mut buttons = vec![];
        for part in line.split_whitespace() {
            match Token::from_str(part) {
                Token::Indicators => {}
                Token::Button(button) => {
                    let mut light_bits = 0;
                    for light in button {
                        light_bits |= 1 << light;
                    }
                    buttons.push(light_bits);
                }
                Token::Joltage(goal) => {
                    joltage_goal = goal;
                }
            }
        }
        Self {
            joltage_goal: MatrixXx1::from_fn(joltage_goal.len(), |i, _| joltage_goal[i]),
            buttons: DMatrix::from_fn(joltage_goal.len(), buttons.len(), |j, b| {
                ((buttons[b] >> j) & 1) as f64
            }),
        }
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in 0..self.buttons.ncols() {
            write!(f, "(")?;
            let mut comma = "";
            for j in 0..self.buttons.nrows() {
                if *self.buttons.get((j, b)).unwrap() > 0.0 {
                    write!(f, "{comma}{j}")?;
                    comma = ",";
                }
            }
            write!(f, ") ")?;
        }

        write!(f, "{{")?;
        let mut comma = "";
        for goal in &self.joltage_goal {
            write!(f, "{comma}{goal}")?;
            comma = ",";
        }
        write!(f, "}}")
    }
}

enum Token {
    Indicators,
    Button(Vec<usize>),
    Joltage(Vec<f64>),
}

impl Token {
    fn from_str(part: &str) -> Token {
        let c = part.chars().next().unwrap();
        match c {
            '[' => Token::Indicators,
            '(' => Token::Button(parse_button(part)),
            '{' => Token::Joltage(parse_joltage(part)),
            _ => panic!("Unexpected character: {c}"),
        }
    }
}

fn parse_button(part: &str) -> Vec<usize> {
    part.trim_matches('(')
        .trim_end_matches(')')
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect()
}

fn parse_joltage(part: &str) -> Vec<f64> {
    part.trim_matches('{')
        .trim_end_matches('}')
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect()
}

fn solve_machine(machine: &Machine) -> MatrixXx1<f64> {
    let Machine {
        joltage_goal,
        buttons,
    } = machine;
    // Assume there are J joltage_goals and B buttons.
    // buttons is a JxB matrix
    // presses is a Bx1 matrix
    // joltage_goal is a Jx1 matrix
    // buttons * presses = joltage_goal
    lstsq::lstsq(&buttons, &joltage_goal, 1e-14)
        .unwrap()
        .solution
}

fn main() {
    let mut machines = vec![];
    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            machines.push(Machine::from_str(&line));
        }
    }

    let mut total_presses = 0.0;
    for machine in machines {
        let solution = solve_machine(&machine);
        let num_presses = solution.sum();
        println!("Solved {machine}: {solution}");
        println!("Adds up to {num_presses} presses.");
        total_presses += num_presses;
    }
    println!("Total presses: {total_presses}");
}
