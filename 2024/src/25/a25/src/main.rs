use std::io::Read;

enum Graphic {
    Lock([i32; 5]),
    Key([i32; 5]),
}

fn parse_graphic(graphic: &str) -> Graphic {
    let mut cols = [-1; 5];
    for line in graphic.split('\n') {
        for (i, c) in line.bytes().enumerate() {
            match c {
                b'#' => {
                    cols[i] += 1;
                }
                b'.' => {}
                c => panic!("Unexpected character: {c}"),
            }
        }
    }
    if graphic.starts_with("#####") {
        Graphic::Lock(cols)
    } else {
        assert!(graphic.starts_with("....."));
        Graphic::Key(cols)
    }
}

fn fits(lock: &[i32; 5], key: &[i32; 5]) -> bool {
    (0..5).all(|i| lock[i] + key[i] <= 5)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut locks = vec![];
    let mut keys = vec![];
    for graphic in input.split("\n\n") {
        match parse_graphic(graphic) {
            Graphic::Lock(cols) => locks.push(cols),
            Graphic::Key(cols) => keys.push(cols),
        }
    }

    let mut num_fits = 0;
    for lock in &locks {
        for key in &keys {
            if fits(lock, key) {
                num_fits += 1;
            }
        }
    }

    println!("{num_fits} fit together");
}
