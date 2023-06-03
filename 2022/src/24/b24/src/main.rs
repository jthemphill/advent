use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::fmt;
use std::io::{stdin, BufRead};

type Point = (usize, usize);

macro_rules! debug_println {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Blizzard {
    point: Point,
    dir: Dir,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    fn apply(&self, point: Point) -> Point {
        match self {
            Dir::U => (point.0, point.1 - 1),
            Dir::D => (point.0, point.1 + 1),
            Dir::L => (point.0 - 1, point.1),
            Dir::R => (point.0 + 1, point.1),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Board {
    width: usize,
    height: usize,
    start_x: usize,
    goal_x: usize,
    blizzards: Vec<Vec<Blizzard>>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Explorer {
    minute: usize,
    point: Point,
}

impl Board {
    fn new(bytes: &Vec<Vec<u8>>) -> Self {
        assert!(bytes.len() > 2);
        assert!(bytes[0].len() > 2);

        let start_x = bytes
            .first()
            .unwrap()
            .iter()
            .enumerate()
            .find(|(_, byte)| **byte == b'.')
            .unwrap()
            .0;
        let goal_x = bytes
            .last()
            .unwrap()
            .iter()
            .enumerate()
            .find(|(_, byte)| **byte == b'.')
            .unwrap()
            .0;

        let mut initial_blizzards = vec![];
        for (y, line) in bytes.iter().enumerate() {
            assert!(*line.first().unwrap() == b'#');
            assert!(*line.last().unwrap() == b'#');

            for (x, byte) in line.iter().enumerate() {
                match byte {
                    b'#' => {}
                    b'.' => {}
                    b'^' => initial_blizzards.push(Blizzard {
                        point: (x, y),
                        dir: Dir::U,
                    }),
                    b'v' => initial_blizzards.push(Blizzard {
                        point: (x, y),
                        dir: Dir::D,
                    }),
                    b'<' => initial_blizzards.push(Blizzard {
                        point: (x, y),
                        dir: Dir::L,
                    }),
                    b'>' => initial_blizzards.push(Blizzard {
                        point: (x, y),
                        dir: Dir::R,
                    }),
                    _ => panic!("Unexpected byte: {byte}"),
                }
            }
        }

        Self {
            width: bytes.first().unwrap().len(),
            height: bytes.len(),
            start_x,
            goal_x,
            blizzards: vec![initial_blizzards],
        }
    }

    fn initial_explorer(&self) -> Explorer {
        Explorer {
            point: (self.start_x, 0),
            minute: 0,
        }
    }

    fn run_until(&mut self, minute: usize) {
        while self.blizzards.len() <= minute {
            self.blizzards.push(
                self.blizzards
                    .last()
                    .unwrap()
                    .iter()
                    .map(|blizzard| self.move_blizzard(*blizzard))
                    .collect(),
            );
        }
    }

    fn move_explorer(&mut self, explorer: Explorer, choice: Option<Dir>) -> Option<Explorer> {
        self.run_until(explorer.minute + 1);

        let (new_x, new_y) = choice
            .map(|dir| dir.apply(explorer.point))
            .unwrap_or(explorer.point);

        // Bounds check
        if new_x == 0
            || new_x >= self.width - 1
            || (new_y == 0 && new_x != self.start_x)
            || (new_y == self.height - 1 && new_x != self.goal_x)
            || new_y >= self.height
        {
            return None;
        }

        // Collision check
        if self.blizzards[explorer.minute + 1]
            .iter()
            .any(|blizzard| blizzard.point == (new_x, new_y))
        {
            return None;
        }

        Some(Explorer {
            minute: explorer.minute + 1,
            point: (new_x, new_y),
        })
    }

    fn move_blizzard(&self, blizzard: Blizzard) -> Blizzard {
        let (mut new_x, mut new_y) = blizzard.dir.apply(blizzard.point);

        if new_x == 0 {
            new_x = self.width - 2;
        } else if new_x >= self.width - 1 {
            new_x = 1;
        }

        if new_y == 0 {
            new_y = self.height - 2;
        } else if new_y >= self.height - 1 {
            new_y = 1;
        }

        Blizzard {
            point: (new_x, new_y),
            ..blizzard
        }
    }

    fn manhattan_to_goal(&self, explorer: Explorer) -> usize {
        let (x, y) = explorer.point;
        let dx = self.goal_x.max(x) - self.goal_x.min(x);
        let dy = self.height - 1 - y;
        explorer.minute + dx + dy
    }

    fn manhattan_to_start(&self, explorer: Explorer) -> usize {
        let (x, y) = explorer.point;
        let dx = self.start_x.max(x) - self.start_x.min(x);
        let dy = y - 0;
        explorer.minute + dx + dy
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", render_state(self, None))
    }
}

fn render_state(board: &Board, explorer: Option<Explorer>) -> String {
    let mut out = Vec::with_capacity(board.height * (board.width + 1));
    for x in 0..board.width {
        if x == board.start_x {
            out.push(b'.');
        } else {
            out.push(b'#');
        }
    }
    out.push(b'\n');
    for _ in 1..(board.height - 1) {
        out.push(b'#');
        for _ in 1..board.width - 1 {
            out.push(b'.');
        }
        out.push(b'#');
        out.push(b'\n');
    }
    for x in 0..board.width {
        if x == board.goal_x {
            out.push(b'.');
        } else {
            out.push(b'#');
        }
    }
    out.push(b'\n');

    let minute = match explorer {
        Some(Explorer { point: _, minute }) => minute,
        _ => 0,
    };

    // Print blizzards, handling collisions by displaying the number of blizzards (if there are 2-9 blizzards)
    for blizzard in &board.blizzards[minute] {
        let (x, y) = blizzard.point;
        out[y * (board.width + 1) + x] = match out[y * (board.width + 1) + x] {
            b'.' => match blizzard.dir {
                Dir::U => b'^',
                Dir::D => b'v',
                Dir::L => b'<',
                Dir::R => b'>',
            },
            b'^' | b'v' | b'<' | b'>' => b'2',
            b'2'..=b'8' => out[y * (board.width + 1) + x] + 1,
            _ => b'@',
        }
    }

    // Print explorer
    if let Some(explorer) = explorer {
        out[explorer.point.1 * (board.width + 1) + explorer.point.0] = b'E';
    }

    String::from_utf8(out).unwrap()
}

fn a_star_to_goal(board: &mut Board, initial_explorer: Explorer) -> Option<Vec<Explorer>> {
    let mut seen = HashSet::new();
    let mut frontier = BinaryHeap::new();
    frontier.push(Reverse((
        board.manhattan_to_goal(initial_explorer),
        vec![initial_explorer],
    )));

    while let Some(Reverse((_, path))) = frontier.pop() {
        let explorer = *path.last().unwrap();
        if !seen.insert(explorer) {
            continue;
        }

        if explorer.point == (board.goal_x, board.height - 1) {
            return Some(path);
        }

        let mut choices = vec![None];
        let (x, y) = explorer.point;
        if x > 0 {
            choices.push(Some(Dir::L));
        }
        if x < board.width - 1 {
            choices.push(Some(Dir::R));
        }
        if y > 0 {
            choices.push(Some(Dir::U));
        }
        if y < board.height - 1 {
            choices.push(Some(Dir::D));
        }

        for choice in choices {
            if let Some(new_explorer) = board.move_explorer(explorer, choice) {
                let mut new_path = path.clone();
                new_path.push(new_explorer);
                frontier.push(Reverse((board.manhattan_to_goal(new_explorer), new_path)));
            }
        }
    }

    None
}

fn a_star_to_start(board: &mut Board, initial_explorer: Explorer) -> Option<Vec<Explorer>> {
    let mut seen = HashSet::new();
    let mut frontier = BinaryHeap::new();
    frontier.push(Reverse((
        board.manhattan_to_start(initial_explorer),
        vec![initial_explorer],
    )));

    while let Some(Reverse((_, path))) = frontier.pop() {
        let explorer = *path.last().unwrap();
        if !seen.insert(explorer) {
            continue;
        }

        if explorer.point == (board.start_x, 0) {
            return Some(path);
        }

        let mut choices = vec![None];
        let (x, y) = explorer.point;
        if x > 0 {
            choices.push(Some(Dir::L));
        }
        if x < board.width - 1 {
            choices.push(Some(Dir::R));
        }
        if y > 0 {
            choices.push(Some(Dir::U));
        }
        if y < board.height - 1 {
            choices.push(Some(Dir::D));
        }

        for choice in choices {
            if let Some(new_explorer) = board.move_explorer(explorer, choice) {
                let mut new_path = path.clone();
                new_path.push(new_explorer);
                frontier.push(Reverse((board.manhattan_to_start(new_explorer), new_path)));
            }
        }
    }

    None
}

fn main() {
    let input_bytes: Vec<Vec<u8>> = stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().as_bytes().into())
        .collect();
    let mut board = Board::new(&input_bytes);

    debug_println!("Initial state:");
    debug_println!("{board}");

    let initial_explorer = board.initial_explorer();
    let path_to_goal = a_star_to_goal(&mut board, initial_explorer).unwrap();
    println!("{} minutes to goal", path_to_goal.last().unwrap().minute);
    let path_to_start = a_star_to_start(&mut board, *path_to_goal.last().unwrap()).unwrap();
    println!(
        "{} minutes back to start",
        path_to_start.last().unwrap().minute
    );
    let path_back_to_goal = a_star_to_goal(&mut board, *path_to_start.last().unwrap()).unwrap();
    println!(
        "{} minutes back to goal",
        path_back_to_goal.last().unwrap().minute
    );
}
