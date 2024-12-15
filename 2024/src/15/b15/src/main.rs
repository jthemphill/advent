use std::{ascii::escape_default, io::Read};

fn apply_dir(pos: (usize, usize), dir: u8) -> (usize, usize) {
    let (mut x, mut y) = pos;
    match dir {
        b'^' => {
            y -= 1;
        }
        b'>' => {
            x += 1;
        }
        b'v' => {
            y += 1;
        }
        b'<' => {
            x -= 1;
        }
        _ => {
            panic!("Unexpected direction: {dir}");
        }
    }
    (x, y)
}

struct Grid {
    bytes: Vec<u8>,
    width: usize,
    height: usize,
    bot_pos: (usize, usize),
}

impl Grid {
    fn new(bytes: Vec<u8>) -> Self {
        let width = bytes.iter().position(|&c| c == b'\n').unwrap();
        let height = (bytes.len() + 1) / (width + 1);

        let bot_pos = bytes.iter().position(|&c| c == b'@').unwrap();
        let bot_y = bot_pos / (width + 1);
        let bot_x = bot_pos - bot_y * (width + 1);

        Self {
            bytes,
            width,
            height,
            bot_pos: (bot_x, bot_y),
        }
    }

    fn get(&self, (x, y): (usize, usize)) -> u8 {
        self.bytes[y * (self.width + 1) + x]
    }

    fn get_mut(&mut self, (x, y): (usize, usize)) -> &mut u8 {
        &mut self.bytes[y * (self.width + 1) + x]
    }

    fn move_bot(&mut self, src: (usize, usize), dir: u8) {
        assert_eq!(self.bot_pos, src);
        assert_eq!(self.get(src), b'@');

        let dst = apply_dir(src, dir);
        let byte_at_dst = self.get(dst);
        let can_move = match byte_at_dst {
            b'#' => false,
            b'.' => true,
            b'[' => {
                if self.can_move_box(dst, dir) {
                    self.move_box(dst, dir);
                    true
                } else {
                    false
                }
            }
            b']' => {
                if self.can_move_box((dst.0 - 1, dst.1), dir) {
                    self.move_box((dst.0 - 1, dst.1), dir);
                    true
                } else {
                    false
                }
            }
            _ => {
                panic!(
                    "Unexpected character {} found in {:?} -> {} -> {:?}",
                    escape_default(byte_at_dst),
                    src,
                    escape_default(dir),
                    dst
                );
            }
        };
        if can_move {
            *self.get_mut(src) = b'.';
            *self.get_mut(dst) = b'@';
            self.bot_pos = dst;
        }
    }

    fn can_move_box(&self, left_src: (usize, usize), dir: u8) -> bool {
        let right_src = (left_src.0 + 1, left_src.1);

        if self.get(left_src) != b'[' {
            panic!("Not [: {}", escape_default(self.get(left_src)));
        }
        if self.get(right_src) != b']' {
            panic!("Not ]: {}", escape_default(self.get(right_src)));
        }

        let left_dst = apply_dir(left_src, dir);
        let right_dst = apply_dir(right_src, dir);

        let valid_dst = |dst: (usize, usize)| -> bool {
            match self.get(dst) {
                b'#' => false,
                b'[' => self.can_move_box(dst, dir),
                b']' => self.can_move_box((dst.0 - 1, dst.1), dir),
                b'.' => true,
                _ => panic!("Unexpected byte {}", escape_default(self.get(dst))),
            }
        };

        match dir {
            b'^' | b'v' => valid_dst(left_dst) && valid_dst(right_dst),
            b'>' => valid_dst(right_dst),
            b'<' => valid_dst(left_dst),
            _ => panic!("unexpected dir: {}", escape_default(dir)),
        }
    }

    fn move_box(&mut self, left_src: (usize, usize), dir: u8) {
        if self.get(left_src) != b'[' {
            panic!("Expected [, got {}", escape_default(self.get(left_src)));
        }
        assert_eq!(self.get(left_src), b'[');
        let right_src = (left_src.0 + 1, left_src.1);
        assert_eq!(self.get((left_src.0 + 1, left_src.1)), b']');

        let left_dst = apply_dir(left_src, dir);
        let right_dst = apply_dir(right_src, dir);

        match dir {
            b'^' | b'v' => {
                for dst in [left_dst, right_dst] {
                    match self.get(dst) {
                        b'[' => {
                            self.move_box(dst, dir);
                        }
                        b']' => {
                            self.move_box((dst.0 - 1, dst.1), dir);
                        }
                        b'.' => {}
                        _ => {
                            panic!("Unexpected char: {}", escape_default(self.get(dst)))
                        }
                    }
                    assert_eq!(self.get(dst), b'.');
                }

                *self.get_mut(left_src) = b'.';
                *self.get_mut(right_src) = b'.';
            }
            b'>' => {
                assert_eq!(self.get(left_dst), b']');
                match self.get(right_dst) {
                    b'[' => {
                        self.move_box(right_dst, dir);
                    }
                    b'.' => {}
                    _ => {
                        panic!(
                            "Unexpected char: {} while moving {:?} -> {:?}",
                            escape_default(self.get(right_dst)),
                            right_src,
                            right_dst,
                        )
                    }
                }
                assert_eq!(self.get(right_dst), b'.');

                *self.get_mut(left_src) = b'.';
            }
            b'<' => {
                assert_eq!(self.get(right_dst), b'[');
                match self.get(left_dst) {
                    b']' => {
                        self.move_box((left_dst.0 - 1, left_dst.1), dir);
                    }
                    b'.' => {}
                    _ => {
                        panic!("Unexpected char: {}", escape_default(self.get(left_dst)))
                    }
                }
                assert_eq!(self.get(left_dst), b'.');

                *self.get_mut(right_src) = b'.';
            }
            _ => panic!("unexpected dir: {}", escape_default(dir)),
        };
        *self.get_mut(left_dst) = b'[';
        *self.get_mut(right_dst) = b']';
    }

    fn gps(&self) -> usize {
        let mut score = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get((x, y)) == b'[' {
                    let box_score = 100 * y + x;
                    println!("{box_score}");
                    score += box_score;
                }
            }
        }
        score
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut stages = input.split("\n\n");
    let original_grid = stages.next().unwrap().as_bytes().to_vec();
    let mut bigger_grid = Vec::with_capacity(original_grid.len() * 2);
    for &c in original_grid.iter() {
        match c {
            b'#' => {
                bigger_grid.push(b'#');
                bigger_grid.push(b'#');
            }
            b'O' => {
                bigger_grid.push(b'[');
                bigger_grid.push(b']');
            }
            b'.' => {
                bigger_grid.push(b'.');
                bigger_grid.push(b'.');
            }
            b'@' => {
                bigger_grid.push(b'@');
                bigger_grid.push(b'.');
            }
            _ => {
                bigger_grid.push(c);
            }
        }
    }
    let mut grid = Grid::new(bigger_grid);
    let directions = stages.next().unwrap();

    for &dir in directions.as_bytes().iter() {
        if dir == b'\n' {
            continue;
        }
        grid.move_bot(grid.bot_pos, dir);
        // println!("{}", String::from_utf8(grid.bytes.clone()).unwrap());
    }
    println!("{}", grid.gps());
}
