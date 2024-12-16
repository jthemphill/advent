use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    io::Read,
};

type Pos = (usize, usize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Dir {
    N = 0,
    E = 1,
    S = 2,
    W = 3,
}

impl Dir {
    fn cw(&self) -> Self {
        match self {
            Self::N => Self::E,
            Self::E => Self::S,
            Self::S => Self::W,
            Self::W => Self::N,
        }
    }

    fn ccw(&self) -> Self {
        match self {
            Self::N => Self::W,
            Self::W => Self::S,
            Self::S => Self::E,
            Self::E => Self::N,
        }
    }

    fn apply(&self, (x, y): Pos) -> Pos {
        match self {
            Self::N => (x, y - 1),
            Self::W => (x - 1, y),
            Self::S => (x, y + 1),
            Self::E => (x + 1, y),
        }
    }
}

struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(data: Vec<u8>) -> Self {
        let width = data.iter().position(|&c| c == b'\n').unwrap();
        let height = (data.len() + 1) / (width + 1);
        Self {
            data,
            width,
            height,
        }
    }

    fn get(&self, pos: Pos) -> u8 {
        let (x, y) = pos;
        self.data[y * (self.width + 1) + x]
    }

    fn start(&self) -> Pos {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get((x, y)) == b'S' {
                    return (x, y);
                }
            }
        }
        panic!("No 'S' found");
    }

    fn end(&self) -> Pos {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get((x, y)) == b'E' {
                    return (x, y);
                }
            }
        }
        panic!("No 'E' found");
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct State {
    score: usize,
    pos: Pos,
    dir: Dir,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn a_star(grid: &Grid) -> usize {
    let mut seen = HashSet::new();
    let mut frontier = BinaryHeap::new();
    frontier.push(State {
        pos: grid.start(),
        dir: Dir::E,
        score: 0,
    });

    let end = grid.end();

    while let Some(state) = frontier.pop() {
        if state.pos == end {
            return state.score;
        }
        if grid.get(state.pos) == b'#' {
            continue;
        }
        if !seen.insert((state.pos, state.dir)) {
            continue;
        }

        frontier.push(State {
            pos: state.dir.apply(state.pos),
            dir: state.dir,
            score: state.score + 1,
        });
        frontier.push(State {
            pos: state.pos,
            dir: state.dir.cw(),
            score: state.score + 1000,
        });
        frontier.push(State {
            pos: state.pos,
            dir: state.dir.ccw(),
            score: state.score + 1000,
        });
    }
    panic!("No path to end found");
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = input.as_bytes().to_vec();

    let grid = Grid::new(input);
    println!("{}", a_star(&grid));
}
