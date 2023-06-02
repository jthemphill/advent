use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::BufRead;

macro_rules! debug_println {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}

#[derive(Debug)]
struct Board {
    elves: HashSet<Elf>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            elves: HashSet::default(),
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        out.reserve_exact(self.num_tiles_in_area());
        for y in (self.min_y)..=(self.max_y) {
            for x in (self.min_x)..=(self.max_x) {
                if self.elves.contains(&Elf { x, y }) {
                    out.push('#');
                } else {
                    out.push('.');
                }
            }
            out.push('\n');
        }
        write!(f, "{out}")
    }
}

impl Board {
    fn insert(&mut self, elf: Elf) {
        self.min_x = self.min_x.min(elf.x);
        self.max_x = self.max_x.max(elf.x);
        self.min_y = self.min_y.min(elf.y);
        self.max_y = self.max_y.max(elf.y);
        assert!(self.elves.insert(elf));
    }

    fn next_generation(&mut self, round: usize) -> Self {
        let mut proposed_moves: HashMap<Elf, Vec<Elf>> = HashMap::new();
        for elf in self.elves.iter() {
            if let Some(next_elf) = elf.get_proposed_move(&self.elves, round) {
                proposed_moves.entry(next_elf).or_default().push(*elf);
            } else {
                proposed_moves.entry(*elf).or_default().push(*elf);
            }
        }
        let mut next_gen = Self::default();
        for (destination, moving_elves) in proposed_moves.into_iter() {
            if moving_elves.len() == 1 {
                next_gen.insert(destination);
            } else {
                debug_println!("Collision between {:?} at {}", moving_elves, destination);
                for elf in moving_elves {
                    next_gen.insert(elf);
                }
            }
        }
        assert_eq!(next_gen.elves.len(), self.elves.len());
        next_gen
    }

    fn num_tiles_in_area(&self) -> usize {
        ((self.max_x - self.min_x + 1) * (self.max_y - self.min_y + 1)) as usize
    }

    fn num_empty_ground_tiles(&self) -> usize {
        self.num_tiles_in_area() - self.elves.len()
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Elf {
    x: i64,
    y: i64,
}

impl fmt::Display for Elf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Elf {
    fn can_stand_still(&self, elves: &HashSet<Elf>) -> bool {
        (-1..=1).all(|dx| {
            (-1..=1).all(|dy| {
                (dx == 0 && dy == 0)
                    || !elves.contains(&Elf {
                        x: self.x + dx,
                        y: self.y + dy,
                    })
            })
        })
    }

    fn can_move_north(&self, elves: &HashSet<Elf>) -> bool {
        (-1..=1).all(|dx| {
            !elves.contains(&Elf {
                x: self.x + dx,
                y: self.y - 1,
            })
        })
    }

    fn can_move_south(&self, elves: &HashSet<Elf>) -> bool {
        (-1..=1).all(|dx| {
            !elves.contains(&Elf {
                x: self.x + dx,
                y: self.y + 1,
            })
        })
    }

    fn can_move_west(&self, elves: &HashSet<Elf>) -> bool {
        (-1..=1).all(|dy| {
            !elves.contains(&Elf {
                x: self.x - 1,
                y: self.y + dy,
            })
        })
    }

    fn can_move_east(&self, elves: &HashSet<Elf>) -> bool {
        (-1..=1).all(|dy| {
            !elves.contains(&Elf {
                x: self.x + 1,
                y: self.y + dy,
            })
        })
    }

    fn get_proposed_move(&self, elves: &HashSet<Elf>, round: usize) -> Option<Elf> {
        if self.can_stand_still(elves) {
            debug_println!("{self} doesn't move");
            None
        } else {
            for i in 0..4 {
                match (round + i) % 4 {
                    0 => {
                        if self.can_move_north(elves) {
                            debug_println!("{self} wants to move N");
                            return Some(Elf {
                                x: self.x,
                                y: self.y - 1,
                            });
                        }
                    }
                    1 => {
                        if self.can_move_south(elves) {
                            debug_println!("{self} wants to move S");
                            return Some(Elf {
                                x: self.x,
                                y: self.y + 1,
                            });
                        }
                    }
                    2 => {
                        if self.can_move_west(elves) {
                            debug_println!("{self} wants to move W");
                            return Some(Elf {
                                x: self.x - 1,
                                y: self.y,
                            });
                        }
                    }
                    3 => {
                        if self.can_move_east(elves) {
                            debug_println!("{self} wants to move E");
                            return Some(Elf {
                                x: self.x + 1,
                                y: self.y,
                            });
                        }
                    }
                    _ => unreachable!("Modulo 4"),
                }
            }
            debug_println!("{self} cannot move!");
            None
        }
    }
}

fn main() {
    let mut board = Board::default();
    for (row, line) in std::io::stdin().lock().lines().enumerate() {
        for (col, tile) in line.unwrap().as_bytes().iter().enumerate() {
            match tile {
                b'.' => {}
                b'#' => {
                    board.insert(Elf {
                        x: col as i64,
                        y: row as i64,
                    });
                }
                _ => panic!("Unexpected char: {tile}"),
            }
        }
    }
    debug_println!("== Initial State ==");
    debug_println!("{board}");
    for round in 1..=10 {
        board = board.next_generation(round - 1);
        debug_println!("== End of Round {round} ==");
        debug_println!("{board}");
    }
    println!(
        "Number of empty ground tiles: {}",
        board.num_empty_ground_tiles(),
    );
}
