fn is_symbol(c: u8) -> bool {
    c != b'.' && !c.is_ascii_digit()
}

fn symbol_adjacent_number(map: &Vec<Vec<u8>>, r: usize, mut c: usize) -> (i32, usize) {
    let mut total: i32 = 0;
    let mut valid = false;
    while c < map[r].len() && map[r][c].is_ascii_digit() {
        total *= 10;
        total += (map[r][c] - b'0') as i32;
        if !valid {
            for r in [r.wrapping_sub(1), r, r + 1] {
                if r >= map.len() {
                    continue;
                }
                for c in [c.wrapping_sub(1), c, c + 1] {
                    if c >= map[r].len() {
                        continue;
                    }
                    if is_symbol(map[r][c]) {
                        valid = true;
                    }
                }
            }
        }
        c += 1;
    }
    if valid {
        (total, c)
    } else {
        (0, c)
    }
}

fn main() {
    let mut map = vec![];
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        map.push(line.trim().as_bytes().to_vec());
    }

    let mut sum = 0;
    for (r, row) in map.iter().enumerate() {
        let mut c = 0;
        while c < row.len() {
            if row[c].is_ascii_digit() {
                let (num, new_c) = symbol_adjacent_number(&map, r, c);
                c = new_c;
                sum += num;
            } else {
                c += 1;
            }
        }
    }
    println!("Sum: {}", sum);
}
