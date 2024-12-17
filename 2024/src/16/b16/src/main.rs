use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    pos: Pos,
    dir: Dir,
    path: Vec<Pos>,
    score: usize,
}

impl State {
    fn new(pos: Pos, dir: Dir, path: Vec<Pos>, score: usize) -> Self {
        Self {
            pos,
            dir,
            path,
            score,
        }
    }
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

fn a_star(grid: &Grid) -> (usize, HashSet<Pos>) {
    let mut frontier = BinaryHeap::new();
    let mut seen = HashMap::new();

    let start = grid.start();
    let end = grid.end();

    frontier.push(State::new(start, Dir::E, vec![start], 0));

    let mut best_score = std::usize::MAX;
    let mut seats = HashSet::new();
    while let Some(state) = frontier.pop() {
        if grid.get(state.pos) == b'#' {
            continue;
        }

        let score_at_state = seen
            .entry((state.pos, state.dir))
            .or_insert(state.score.min(best_score));
        if state.score > *score_at_state {
            continue;
        } else {
            *score_at_state = state.score;
        }

        if state.pos == end {
            best_score = state.score;
            for &seat in &state.path {
                seats.insert(seat);
            }
            println!("{} {} {}", frontier.len(), seen.len(), seats.len());
            continue;
        }

        // if !seen.insert((state.pos, state.dir)) {
        //     if state.path.iter().all(|pos| seats.contains(pos)) {
        //         continue;
        //     }
        // }

        for dir in [state.dir.cw(), state.dir.ccw()] {
            frontier.push(State::new(
                state.pos,
                dir,
                state.path.clone(),
                state.score + 1000,
            ));
        }
        let mut forward_path = state.path;
        forward_path.push(state.dir.apply(state.pos));
        frontier.push(State::new(
            state.dir.apply(state.pos),
            state.dir,
            forward_path,
            state.score + 1,
        ));
    }
    (best_score, seats)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = input.as_bytes().to_vec();

    let grid = Grid::new(input);
    let (best_score, seats) = a_star(&grid);
    println!("score: {best_score}");

    for y in 0..grid.height {
        let mut line = Vec::new();
        for x in 0..grid.width {
            if seats.contains(&(x, y)) {
                line.push(b'O');
            } else {
                line.push(grid.get((x, y)));
            }
        }
        println!("{}", String::from_utf8(line).unwrap());
    }

    println!("seats: {}", seats.len());
}
