use std::{io::BufRead, str::FromStr};

#[derive(Debug)]
enum Operator {
    Plus,
    Times,
}

struct OpPos {
    op: Operator,
    start: usize,
    end: usize,
}

fn main() {
    let mut lines = Vec::new();
    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            lines.push(line.into_bytes());
        }
    }

    let width = lines[0].len();

    let mut operators = Vec::new();
    for start in (0..lines[lines.len() - 1].len()).rev() {
        let char = lines[lines.len() - 1][start];
        let end = operators
            .last()
            .map(|op_pos: &OpPos| op_pos.start)
            .unwrap_or(width);
        match char {
            b'+' => operators.push(OpPos {
                op: Operator::Plus,
                start,
                end,
            }),
            b'*' => operators.push(OpPos {
                op: Operator::Times,
                start,
                end,
            }),
            _ => {}
        };
    }

    let mut answer_total = 0;
    for OpPos { start, end, op } in operators {
        let mut nums = Vec::new();
        for c in (start..end).rev() {
            let mut num = 0;
            for r in 0..lines.len() - 1 {
                if b'0' <= lines[r][c] && lines[r][c] <= b'9' {
                    num *= 10;
                    num += (lines[r][c] - b'0') as i64;
                }
            }
            println!("{num}");
            if num != 0 {
                nums.push(num);
            }
        }
        let answer: i64 = match op {
            Operator::Plus => nums.iter().sum::<i64>(),
            Operator::Times => nums.iter().product::<i64>(),
        };
        println!("answer: {answer}");
        answer_total += answer;
    }
    println!("total: {answer_total}");
}
