use std::{io::BufRead, str::FromStr};

#[derive(Debug)]
enum Operator {
    Plus,
    Times,
}

#[derive(Debug)]
enum Value {
    Op(Operator),
    Num(i64),
}

#[derive(Debug)]
struct ParseValueError;

impl FromStr for Value {
    type Err = ParseValueError;

    fn from_str(token: &str) -> Result<Self, Self::Err> {
        Ok(match token {
            "+" => Self::Op(Operator::Plus),
            "*" => Self::Op(Operator::Times),
            _ => Self::Num(token.parse().map_err(|_| ParseValueError)?),
        })
    }
}

fn main() {
    let mut row_nums = Vec::new();
    let mut value_ops = Vec::new();
    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            for token in line.split_whitespace() {
                match Value::from_str(token).unwrap() {
                    Value::Op(op) => value_ops.push(op),
                    Value::Num(n) => row_nums.push(n),
                }
            }
        }
    }

    let mut answer_total = 0;
    for i in 0..value_ops.len() {
        let mut j = i;
        let mut answer = match value_ops[i] {
            Operator::Plus => 0,
            Operator::Times => 1,
        };
        while j < row_nums.len() {
            let num = row_nums[j];
            println!("Column {i}: {num}");
            match value_ops[i] {
                Operator::Plus => answer += num,
                Operator::Times => answer *= num,
            };

            j += value_ops.len();
        }
        println!("{answer}");
        answer_total += answer;
    }
    println!("Total: {answer_total}");
}
