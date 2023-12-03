use std::collections::HashSet;

fn get_num(row: &Vec<u8>, min_c: usize, max_c: usize) -> i32 {
    let mut c = min_c;
    let mut num = 0;
    while c < max_c {
        num *= 10;
        num += (row[c] - b'0') as i32;
        c += 1;
    }
    num
}

fn get_num_span(map: &Vec<Vec<u8>>, r: usize, c: usize) -> (usize, usize, usize) {
    let mut min_c = c;
    while min_c > 0 && map[r][min_c - 1].is_ascii_digit() {
        min_c -= 1;
    }

    let mut max_c = c;
    while max_c < map[r].len() && map[r][max_c].is_ascii_digit() {
        max_c += 1;
    }

    (r, min_c, max_c)
}

fn get_ratio(map: &Vec<Vec<u8>>, r: usize, c: usize) -> i32 {
    let mut spans = HashSet::new();

    for r in [r.wrapping_sub(1), r, r + 1] {
        if r >= map.len() {
            continue;
        }
        for c in [c.wrapping_sub(1), c, c + 1] {
            if c >= map[r].len() {
                continue;
            }
            if map[r][c].is_ascii_digit() {
                spans.insert(get_num_span(map, r, c));
            }
        }
    }

    if spans.len() != 2 {
        return 0;
    }
    spans.iter().fold(1, |acc, &(r, min_c, max_c)| {
        acc * get_num(&map[r], min_c, max_c)
    })
}

fn main() {
    let mut map = vec![];
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        map.push(line.trim().as_bytes().to_vec());
    }

    let mut sum = 0;
    for (r, row) in map.iter().enumerate() {
        for (c, char) in row.iter().enumerate() {
            if *char == b'*' {
                sum += get_ratio(&map, r, c);
            }
        }
    }
    println!("Sum: {}", sum);
}
