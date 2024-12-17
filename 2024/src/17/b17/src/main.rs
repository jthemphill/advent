use regex::Regex;
use std::{collections::BinaryHeap, io::Read};

fn run(mut a: usize, opcodes: &Vec<usize>) -> Vec<usize> {
    let mut b = 0;
    let mut c = 0;

    let mut outputs = vec![];
    let mut instrp = 0;
    while instrp < opcodes.len() {
        let opcode = opcodes[instrp];
        let operand = opcodes[instrp + 1];

        let combo = match operand {
            0 | 1 | 2 | 3 => operand,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!("Combo operand {operand}"),
        };

        match opcode {
            0 => {
                // adv
                let numerator = a;
                let denominator = 2_usize.pow(combo as u32);
                a = numerator / denominator;
            }
            1 => {
                // bxl
                b = b ^ operand;
            }
            2 => {
                // bst
                b = combo % 8;
            }
            3 => {
                // jnz
                if a != 0 {
                    instrp = (operand as usize).wrapping_sub(2);
                }
            }
            4 => {
                // bxc
                b = b ^ c;
            }
            5 => {
                // out
                outputs.push(combo % 8);
            }
            6 => {
                // bdv
                let numerator = a;
                let denominator = 2_usize.pow(combo as u32);
                b = numerator / denominator;
            }
            7 => {
                // cdv
                let numerator = a;
                let denominator = 2_usize.pow(combo as u32);
                c = numerator / denominator;
            }
            _ => panic!("Opcode {opcode}"),
        }

        instrp = instrp.wrapping_add(2);
    }
    outputs
}

fn main() {
    let prog_re = Regex::new(r"Program: (.*)").unwrap();

    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut lines = input.split('\n');
    lines.next().unwrap(); // a
    lines.next().unwrap(); // b
    lines.next().unwrap(); // c
    lines.next().unwrap();

    let opcodes_str = prog_re
        .captures(lines.next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    let opcodes: Vec<usize> = opcodes_str.split(',').map(|c| c.parse().unwrap()).collect();
    assert!(opcodes.len() % 2 == 0);

    let mut stack = BinaryHeap::new();
    stack.push((0, 0, 1));
    while let Some((a, attempt, len)) = stack.pop() {
        let out = run(a, &opcodes);
        let mut winning = true;
        for (i, &opcode) in opcodes.iter().skip(opcodes.len() - len).enumerate() {
            if out[i] != opcode {
                winning = false;
            }
        }
        if winning {
            if len == opcodes.len() {
                println!("{opcodes:?} Done! {a} {out:?}");
                break;
            } else {
                println!("{opcodes:?} {a} {out:?}");
                stack.push((a * 8, 0, len + 1));
            }
        }
        if attempt < 8 {
            stack.push((a + 1, attempt + 1, len));
        }
    }
}
