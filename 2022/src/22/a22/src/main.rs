use std::fmt::Display;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Forward(i64),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Facing {
    North,
    South,
    East,
    West,
}

impl Facing {
    fn score(&self) -> usize {
        match self {
            Facing::East => 0,
            Facing::South => 1,
            Facing::West => 2,
            Facing::North => 3,
        }
    }

    fn left(&self) -> Self {
        match self {
            Facing::North => Facing::West,
            Facing::West => Facing::South,
            Facing::South => Facing::East,
            Facing::East => Facing::North,
        }
    }

    fn right(&self) -> Self {
        match self {
            Facing::North => Facing::East,
            Facing::East => Facing::South,
            Facing::South => Facing::West,
            Facing::West => Facing::North,
        }
    }
}

impl Into<u8> for Facing {
    fn into(self) -> u8 {
        match self {
            Facing::North => b'^',
            Facing::South => b'v',
            Facing::East => b'>',
            Facing::West => b'<',
        }
    }
}

struct Board {
    rows: Vec<Vec<u8>>,
    directions: Vec<Direction>,
    x: usize,
    y: usize,
    facing: Facing,
    t: usize,
}

impl Board {
    fn new(rows: Vec<Vec<u8>>, directions: Vec<Direction>) -> Self {
        let x = rows[0]
            .iter()
            .enumerate()
            .find(|(_, c)| **c != b' ')
            .unwrap()
            .0;
        let y = 0;
        let facing = Facing::East;
        let t = 0;
        let mut me = Self {
            rows,
            directions,
            x,
            y,
            facing,
            t,
        };
        me.mark();
        me
    }

    fn run(&mut self) {
        while self.t < self.directions.len() {
            self.step();
        }
    }

    fn step(&mut self) {
        match self.directions[self.t] {
            Direction::Left => {
                self.facing = self.facing.left();
                self.mark();
            }
            Direction::Right => {
                self.facing = self.facing.right();
                self.mark();
            }
            Direction::Forward(n) => {
                for _ in 0..n {
                    if !self.forward() {
                        break;
                    }
                    self.mark();
                }
            }
        }
        self.t += 1;
    }

    fn forward(&mut self) -> bool {
        let (new_x, new_y) = match self.facing {
            Facing::North => {
                let mut new_y = (self.y + self.rows.len() - 1) % self.rows.len();
                while self.get(self.x, new_y).is_none() {
                    new_y = (new_y + self.rows.len() - 1) % self.rows.len();
                }
                (self.x, new_y)
            }
            Facing::South => {
                let mut new_y = (self.y + 1) % self.rows.len();
                while self.get(self.x, new_y).is_none() {
                    new_y = (new_y + 1) % self.rows.len();
                }
                (self.x, new_y)
            }
            Facing::East => {
                let row = &self.rows[self.y];
                let mut new_x = (self.x + 1) % row.len();
                while self.get(new_x, self.y).is_none() {
                    new_x = (new_x + 1) % row.len();
                }
                (new_x, self.y)
            }
            Facing::West => {
                let row = &self.rows[self.y];
                let mut new_x = (self.x + row.len() - 1) % row.len();
                while self.get(new_x, self.y).is_none() {
                    new_x = (new_x + row.len() - 1) % row.len();
                }
                (new_x, self.y)
            }
        };
        if self.get(new_x, new_y).unwrap() != b'#' {
            self.x = new_x;
            self.y = new_y;
            true
        } else {
            false
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<u8> {
        if y >= self.rows.len() {
            return None;
        }
        let row = &self.rows[y];
        if x >= row.len() {
            return None;
        }
        let c = row[x];
        if c == b' ' {
            return None;
        }
        Some(c)
    }

    fn mark(&mut self) {
        let row = &mut self.rows[self.y];
        assert_ne!(row[self.x], b' ');
        assert_ne!(row[self.x], b'#');
        row[self.x] = self.facing.into();
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            write!(f, "{}\n", std::str::from_utf8(row.as_slice()).unwrap())?;
        }
        write!(f, "\n{:?}\n", self.directions)?;
        Ok(())
    }
}

fn main() {
    let mut making_grid = true;
    let mut rows = vec![];
    let mut directions = vec![];
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                making_grid = false;
            } else if making_grid {
                let mut grid_row = vec![];
                for c in line.bytes() {
                    match c {
                        b' ' | b'#' | b'.' => {
                            grid_row.push(c);
                        }
                        _ => panic!("Unexpected character: {}", c),
                    }
                }
                rows.push(grid_row);
            } else {
                let mut steps = 0;
                for c in line.bytes() {
                    match c {
                        b'L' => {
                            if steps != 0 {
                                directions.push(Direction::Forward(steps));
                                steps = 0;
                            }
                            directions.push(Direction::Left);
                        }
                        b'R' => {
                            if steps != 0 {
                                directions.push(Direction::Forward(steps));
                                steps = 0;
                            }
                            directions.push(Direction::Right);
                        }
                        b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                            steps *= 10;
                            steps += (c - b'0') as i64;
                        }
                        _ => panic!("Unexpected direction character: {}", c),
                    }
                }
                if steps != 0 {
                    directions.push(Direction::Forward(steps));
                    steps = 0;
                }
            }
        }
    }
    let mut board = Board::new(rows, directions);
    println!("{}", board);
    board.run();
    println!("{}", board);
    println!(
        "Row: {}, Col: {}, facing: {} ({:?})",
        board.y + 1,
        board.x + 1,
        board.facing.score(),
        board.facing,
    );
    println!(
        "Score: {}",
        1000 * (board.y + 1) + 4 * (board.x + 1) + board.facing.score()
    );
}
