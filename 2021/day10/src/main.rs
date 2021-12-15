use std::io::prelude::*;

fn corrupt_score(line: &str) -> usize {
    let mut levels = vec![];
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => levels.push(c),
            ')' => {
                if levels.pop() != Some('(') {
                    return 3;
                }
            }
            ']' => {
                if levels.pop() != Some('[') {
                    return 57;
                }
            }
            '}' => {
                if levels.pop() != Some('{') {
                    return 1197;
                }
            }
            '>' => {
                if levels.pop() != Some('<') {
                    return 25137;
                }
            }
            _ => {}
        }
    }
    0
}

fn incomplete_score(line: &str) -> usize {
    let mut levels = vec![];
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => levels.push(c),
            ')' => assert!(levels.pop() == Some('(')),
            ']' => assert!(levels.pop() == Some('[')),
            '}' => assert!(levels.pop() == Some('{')),
            '>' => assert!(levels.pop() == Some('<')),
            _ => {}
        }
    }
    levels.reverse();
    println!("Completion for {}: {:?}", line, levels);
    let mut score = 0;
    for c in levels {
        score *= 5;
        score += match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => 0,
        };
    }
    score
}

fn main() {
    let mut total_corrupt = 0;
    let mut incomplete_scores = vec![];
    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            let line_corrupt_score = corrupt_score(&line);
            if line_corrupt_score > 0 {
                println!("Corrupt score for {}: {}", line, line_corrupt_score);
                total_corrupt += line_corrupt_score;
            } else {
                let line_incomplete_score = incomplete_score(&line);
                println!("Incomplete score for {}: {}", line, line_incomplete_score);
                incomplete_scores.push(line_incomplete_score);
            }
        }
    }
    println!("Total corrupt score: {}", total_corrupt);
    incomplete_scores.sort();
    println!(
        "Middle incomplete score: {}",
        incomplete_scores[incomplete_scores.len() / 2]
    );
}
