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

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    pos: Pos,
    dir: Dir,
    path: Vec<Pos>,
    score: usize,
    estimate: usize,
}

impl State {
    fn new(pos: Pos, dir: Dir, path: Vec<Pos>, score: usize, end: Pos) -> Self {
        let estimate = score + h(pos, end, dir);
        Self {
            pos,
            dir,
            path,
            score,
            estimate,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .estimate
            .cmp(&self.estimate)
            .then_with(|| self.score.cmp(&other.score))
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct HState {
    pos: Pos,
    dir: Dir,
    score: usize,
}

impl Ord for HState {
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

impl PartialOrd for HState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn h(start: Pos, end: Pos, dir: Dir) -> usize {
    let mut seen = HashSet::new();
    let mut frontier = BinaryHeap::new();
    frontier.push(HState {
        pos: start,
        dir,
        score: 0,
    });

    while let Some(state) = frontier.pop() {
        if state.pos == end {
            return state.score;
        }
        if !seen.insert((state.pos, state.dir)) {
            continue;
        }
        for dir in [state.dir.cw(), state.dir.ccw()] {
            frontier.push(HState {
                pos: state.pos,
                dir,
                score: state.score + 1000,
            });
        }
        let (mut x1, mut y1) = state.pos;
        let mut score = state.score;
        let (x2, y2) = end;
        match state.dir {
            Dir::N => {
                if y1 > y2 {
                    score += y1 - y2;
                    y1 = y2;
                }
            }
            Dir::E => {
                if x1 < x2 {
                    score += x2 - x1;
                    x1 = x2;
                }
            }
            Dir::S => {
                if y1 < y2 {
                    score += y2 - y1;
                    y1 = y2;
                }
            }
            Dir::W => {
                if x1 > x2 {
                    score += x1 - x2;
                    x1 = x2;
                }
            }
        };
        frontier.push(HState {
            pos: (x1, y1),
            dir,
            score,
        });
    }
    panic!("panik");
}

fn a_star(grid: &Grid) -> (usize, HashSet<Pos>) {
    let mut frontier = BinaryHeap::new();

    let start = grid.start();
    let end = grid.end();

    frontier.push(State::new(start, Dir::E, vec![start], 0, end));

    let mut best_score = std::usize::MAX;
    let mut seats = HashSet::new();
    while let Some(state) = frontier.pop() {
        assert!(state.estimate >= state.score);
        if state.estimate > best_score || grid.get(state.pos) == b'#' {
            continue;
        }

        if state.pos == end {
            best_score = state.score;
            for &seat in &state.path {
                seats.insert(seat);
            }
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
                end,
            ));
        }
        let mut forward_path = state.path;
        forward_path.push(state.dir.apply(state.pos));
        frontier.push(State::new(
            state.dir.apply(state.pos),
            state.dir,
            forward_path,
            state.score + 1,
            end,
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
