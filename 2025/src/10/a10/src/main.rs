use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fmt::Display,
    io::BufRead,
};

#[derive(Clone, Debug)]
struct Machine {
    goal: u64,
    buttons: Vec<Button>,
}

#[derive(Clone, Copy, Debug)]
struct Button {
    bits: u64,
}

impl Button {
    fn highest_light(&self) -> usize {
        let mut highest_light = 0;
        while self.bits >> highest_light > 0 {
            highest_light += 1;
        }
        highest_light
    }
}

impl Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " (")?;
        let mut comma = "";
        for i in 0..63 {
            if ((self.bits >> i) & 1) == 1 {
                write!(f, "{comma}{i}")?;
                comma = ",";
            }
        }
        write!(f, ")")
    }
}

impl Machine {
    fn from_str(line: &str) -> Self {
        let mut goal = 0;
        let mut buttons = vec![];
        for part in line.split_whitespace() {
            match Token::from_str(part) {
                Token::Indicators(lights) => {
                    for (i, light) in lights.iter().enumerate() {
                        goal |= (if *light { 1 } else { 0 }) << i;
                    }
                }
                Token::Button(button) => {
                    let mut light_bits = 0;
                    for light in button {
                        light_bits |= 1 << light;
                    }
                    buttons.push(Button { bits: light_bits });
                }
                Token::Joltage => {}
            }
        }
        Self { goal, buttons }
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num_lights = self
            .buttons
            .iter()
            .map(|button| button.highest_light())
            .max()
            .unwrap();

        write!(f, "[")?;
        for i in 0..num_lights {
            if ((self.goal >> i) & 1) == 1 {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        write!(f, "]")?;

        for button in &self.buttons {
            write!(f, "{button}")?;
        }
        Ok(())
    }
}

enum Token {
    Indicators(Vec<bool>),
    Button(Vec<usize>),
    Joltage,
}

impl Token {
    fn from_str(part: &str) -> Token {
        let c = part.chars().next().unwrap();
        match c {
            '[' => Token::Indicators(parse_indicators(part)),
            '(' => Token::Button(parse_button(part)),
            '{' => Token::Joltage,
            _ => panic!("Unexpected character: {c}"),
        }
    }
}

fn parse_indicators(part: &str) -> Vec<bool> {
    part.trim_matches('[')
        .trim_end_matches(']')
        .chars()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => panic!("Unexpected light: {c}"),
        })
        .collect()
}

fn parse_button(part: &str) -> Vec<usize> {
    part.trim_matches('(')
        .trim_end_matches(')')
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect()
}

fn solve_machine(machine: &Machine) -> u64 {
    let Machine { goal, buttons } = machine;
    let mut frontier = BinaryHeap::new();
    frontier.push(Reverse((0, 0, 0, 0)));
    let mut seen = HashSet::new();
    while let Some(Reverse((num_presses, state, index, presses))) = frontier.pop() {
        if state == *goal {
            return presses;
        }
        if index >= buttons.len() {
            continue;
        }
        if !seen.insert((state, index)) {
            continue;
        }
        frontier.push(Reverse((
            num_presses + 1,
            state ^ buttons[index].bits,
            index + 1,
            presses | (1 << index),
        )));
        frontier.push(Reverse((num_presses, state, index + 1, presses)));
    }
    panic!("Could not solve {machine}")
}

fn main() {
    let mut machines = vec![];
    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            machines.push(Machine::from_str(&line));
        }
    }

    let mut total_presses = 0;
    for machine in machines {
        let buttons = solve_machine(&machine);
        let button_indices: Vec<_> = (0..64).filter(|i| (buttons >> i) & 1 == 1).collect();
        println!("Solved {machine}: {button_indices:?}");
        total_presses += buttons.count_ones();
    }
    println!("Total presses: {total_presses}");
}
