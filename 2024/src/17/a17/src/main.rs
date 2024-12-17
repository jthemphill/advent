use regex::Regex;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let a_re = Regex::new(r"Register A: (\d+)").unwrap();
    let b_re = Regex::new(r"Register B: (\d+)").unwrap();
    let c_re = Regex::new(r"Register C: (\d+)").unwrap();
    let prog_re = Regex::new(r"Program: (.*)").unwrap();

    let mut lines = input.split('\n');

    let mut a: usize = a_re
        .captures(lines.next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let mut b: usize = b_re
        .captures(lines.next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let mut c: usize = c_re
        .captures(lines.next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    lines.next().unwrap();

    let opcodes_str = prog_re
        .captures(lines.next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    let opcodes: Vec<usize> = opcodes_str.split(',').map(|c| c.parse().unwrap()).collect();
    assert!(opcodes.len() % 2 == 0);

    println!("{a} {b} {c} {opcodes:?}");

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

    let output_str = outputs
        .iter()
        .map(|&n| n.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("{output_str}");
}
