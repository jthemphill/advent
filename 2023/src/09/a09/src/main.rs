fn difference_seq(seq: &[i64]) -> Vec<i64> {
    let mut diff_seq = vec![];
    for i in 1..seq.len() {
        diff_seq.push(seq[i] - seq[i - 1]);
    }
    diff_seq
}

fn main() {
    let mut sequences: Vec<Vec<i64>> = vec![];
    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        sequences.push(
            line.split_ascii_whitespace()
                .map(|token| token.parse().unwrap())
                .collect(),
        );
    }

    let mut extra_sum = 0;
    for seq in sequences {
        let mut seq_stack = vec![seq];
        while seq_stack.last().unwrap().iter().any(|&n| n != 0) {
            seq_stack.push(difference_seq(seq_stack.last().unwrap()));
        }
        for i in (0..(seq_stack.len() - 1)).rev() {
            let val_i = *seq_stack[i].last().unwrap();
            let val_i_next = *seq_stack[i + 1].last().unwrap();
            seq_stack[i].push(val_i + val_i_next);
        }
        extra_sum += seq_stack[0].last().unwrap();
    }
    println!("Sum of extrapolated values: {extra_sum}");
}
