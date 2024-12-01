fn get_start_pos(map: &Vec<Vec<u8>>) -> (usize, usize) {
    for (r, row) in map.iter().enumerate() {
        for (c, char) in row.iter().enumerate() {
            if *char == b'S' {
                return (r, c);
            }
        }
    }
    panic!("No 'S' found");
}

fn get_start_shape(map: &Vec<Vec<u8>>, start_pos: (usize, usize)) -> u8 {
    let (r, c) = start_pos;
    let mut north = false;
    let mut south = false;
    let mut east = false;
    let mut west = false;
    if r > 0 {
        match map[r - 1][c] {
            b'|' | b'7' | b'F' => {
                north = true;
            }
            _ => {}
        };
    }
    if r + 1 < map.len() {
        match map[r + 1][c] {
            b'|' | b'L' | b'J' => {
                south = true;
            }
            _ => {}
        };
    }
    if c > 0 {
        match map[r][c - 1] {
            b'-' | b'L' | b'F' => {
                west = true;
            }
            _ => {}
        };
    }
    if c + 1 < map[r].len() {
        match map[r][c + 1] {
            b'-' | b'J' | b'7' => {
                east = true;
            }
            _ => {}
        };
    }

    if north && east {
        b'L'
    } else if north && south {
        b'|'
    } else if north && west {
        b'J'
    } else if east && south {
        b'F'
    } else if east && west {
        b'-'
    } else if south && west {
        b'7'
    } else {
        panic!(
            "Unexpected directions: {} {} {} {}",
            if north { "N" } else { "" },
            if east { "E" } else { "" },
            if south { "S" } else { "" },
            if west { "W" } else { "" }
        );
    }
}

fn main() {
    let mut map = vec![];
    for line in std::io::stdin().lines() {
        map.push(
            line.unwrap()
                .as_bytes()
                .iter()
                .cloned()
                .collect::<Vec<u8>>(),
        );
    }

    let start_pos = get_start_pos(&map);
    let start_shape = get_start_shape(&map, start_pos);

    println!("S is {} at {:?}", start_shape as char, start_pos);
}
