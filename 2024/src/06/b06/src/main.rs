use std::{collections::HashSet, io::Read};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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

#[derive(Clone, Debug)]
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

    fn next_guard(&self) -> usize {
        match self.dir {
            Dir::N => self.guard.wrapping_sub(self.width + 1),
            Dir::E => self.guard.wrapping_add(1),
            Dir::S => self.guard.wrapping_add(self.width + 1),
            Dir::W => self.guard.wrapping_sub(1),
        }
    }

    fn next(&mut self) -> bool {
        self.data[self.guard] = b'X';
        let new_guard = self.next_guard();
        if new_guard < self.data.len() {
            match self.data[new_guard] {
                b'\n' => {
                    return false;
                }
                b'#' | b'O' => {
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

fn place_obstacle(
    mut grid: Grid,
    mut states: HashSet<(usize, Dir)>,
    obstacle: usize,
) -> Option<Grid> {
    if obstacle >= grid.data.len() {
        return None;
    }

    if grid.data[obstacle] != b'.' {
        return None;
    }

    grid.data[obstacle] = b'O';

    while states.insert((grid.guard, grid.dir)) {
        if !grid.next() {
            return None;
        }
    }
    Some(grid)
}

fn main() {
    let mut data = String::new();
    std::io::stdin().lock().read_to_string(&mut data).unwrap();
    let mut grid = Grid::new(data.as_bytes().to_owned());

    let mut states: HashSet<(usize, Dir)> = HashSet::new();

    let mut obstacles: Vec<(usize, Grid)> = Vec::new();
    loop {
        let obstacle = grid.next_guard();
        if let Some(new_grid) = place_obstacle(grid.clone(), states.clone(), obstacle) {
            obstacles.push((obstacle, new_grid));
        }
        states.insert((grid.guard, grid.dir));
        if !grid.next() {
            break;
        }
    }

    let num_obstacles = obstacles.len();
    println!("Total: {num_obstacles}");
}
