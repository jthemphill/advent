use std::{ascii::escape_default, io::Read};

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

    fn get(&self, x: usize, y: usize) -> u8 {
        self.bytes[y * (self.width + 1) + x]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        &mut self.bytes[y * (self.width + 1) + x]
    }

    fn move_in_dir(&mut self, src: (usize, usize), dir: u8) -> bool {
        let (mut x, mut y) = src;
        let me: u8 = self.get(x, y);
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
        let byte_at_dst = self.get(x, y);
        match byte_at_dst {
            b'#' => false,
            b'.' => {
                *self.get_mut(src.0, src.1) = b'.';
                *self.get_mut(x, y) = me;
                if src == self.bot_pos {
                    self.bot_pos = (x, y);
                }
                true
            }
            b'O' => {
                let could_move_boulder = self.move_in_dir((x, y), dir);
                if could_move_boulder {
                    *self.get_mut(src.0, src.1) = b'.';
                    *self.get_mut(x, y) = me;
                    if src == self.bot_pos {
                        self.bot_pos = (x, y);
                    }
                }
                could_move_boulder
            }
            _ => {
                panic!(
                    "Unexpected character {} found in {:?} -> {} -> {:?}",
                    escape_default(byte_at_dst),
                    src,
                    escape_default(dir),
                    (x, y)
                );
            }
        }
    }

    fn gps(&self) -> usize {
        let mut score = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) == b'O' {
                    score += 100 * y + x;
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
    let mut grid = Grid::new(stages.next().unwrap().as_bytes().to_vec());
    let directions = stages.next().unwrap();

    for &dir in directions.as_bytes().iter() {
        if dir == b'\n' {
            continue;
        }
        grid.move_in_dir(grid.bot_pos, dir);
        // println!("{}", String::from_utf8(grid.bytes.clone()).unwrap());
    }
    println!("{}", grid.gps());
}
