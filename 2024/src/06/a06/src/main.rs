use std::io::Read;

enum Dir {
    N = 0,
    E = 1,
    S = 2,
    W = 3,
}

impl Dir {
    fn repr(&self) -> u8 {
        match self {
            Dir::N => b'^',
            Dir::E => b'>',
            Dir::W => b'<',
            Dir::S => b'V',
        }
    }

    fn cw(&self) -> Self {
        match self {
            Self::N => Self::E,
            Self::E => Self::S,
            Self::S => Self::W,
            Self::W => Self::N,
        }
    }
}

struct Grid {
    data: Vec<u8>,
    width: usize,
    guard: usize,
    dir: Dir,
}

impl Grid {
    fn new(data: Vec<u8>) -> Self {
        let width = data
            .iter()
            .enumerate()
            .find(|(_, &c)| c == b'\n')
            .unwrap()
            .0;

        let guard = data.iter().enumerate().find(|(_, &c)| c == b'^').unwrap().0;

        Self {
            data,
            width,
            guard,
            dir: Dir::N,
        }
    }

    fn next(&mut self) -> bool {
        self.data[self.guard] = b'X';
        let new_guard = match self.dir {
            Dir::N => self.guard.wrapping_sub(self.width + 1),
            Dir::E => self.guard.wrapping_add(1),
            Dir::S => self.guard.wrapping_add(self.width + 1),
            Dir::W => self.guard.wrapping_sub(1),
        };
        if new_guard < self.data.len() {
            match self.data[new_guard] {
                b'#' => {
                    self.dir = self.dir.cw();
                    self.data[self.guard] = self.dir.repr();
                }
                _ => {
                    self.guard = new_guard;
                }
            }
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut data = String::new();
    std::io::stdin().lock().read_to_string(&mut data).unwrap();
    let mut grid = Grid::new(data.as_bytes().to_owned());

    loop {
        if !grid.next() {
            break;
        }
    }
    let num_visited = grid.data.iter().filter(|&&c| c == b'X').count();
    let data = String::from_utf8(grid.data).unwrap();
    println!("{data}");
    println!("{num_visited}");
}
