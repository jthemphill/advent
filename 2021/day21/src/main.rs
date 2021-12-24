use std::io::prelude::*;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Player {
    pos: usize,
    score: usize,
}

impl Player {
    fn new(pos: usize) -> Self {
        Player { pos: pos - 1, score: 0 }
    }

    fn advance(&mut self, roll: usize) {
        self.pos += roll;
        self.pos %= 10;
        self.score += self.position();
    }

    fn position(&self) -> usize {
        self.pos + 1
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Dirac {
    players: [Player; 2],
}

impl Dirac {
    fn new(pos: [usize; 2]) -> Self {
        Dirac {
            players: [Player::new(pos[0]), Player::new(pos[1])],
        }
    }

    fn advance(&self, p: usize, roll: usize) -> Dirac {
        let mut players = self.players;
        players[p].advance(roll);
        Dirac { players }
    }
}

struct DeterministicDie {
    rolls: usize,
}

impl DeterministicDie {
    fn new() -> Self {
        Self { rolls: 0 }
    }

    fn roll(&mut self) -> usize {
        let roll = 3 * self.rolls + 6;
        self.rolls += 3;
        roll
    }

    fn nrolls(&self) -> usize {
        self.rolls
    }
}

fn practice(p1_pos: usize, p2_pos: usize) -> usize {
    let mut dirac = Dirac::new([p1_pos, p2_pos]);
    let mut die = DeterministicDie::new();
    for p in (0..=1).cycle() {
        dirac = dirac.advance(p, die.roll());
        if dirac.players[p].score >= 1000 {
            break;
        }
    }
    dirac.players.iter().map(|p| p.score).min().unwrap() * die.nrolls()
}

type DiracMemo = std::collections::HashMap<Dirac, usize>;

fn real(p1_pos: usize, p2_pos: usize) -> [usize; 2] {
    let mut games = DiracMemo::from([(Dirac::new([p1_pos, p2_pos]), 1)]);
    let rolls: Vec<usize> = (1..=3)
        .flat_map(|a| (1..=3).flat_map(move |b| (1..=3).map(move |c| a + b + c)))
        .collect();
    let mut wins = [0, 0];
    for p in (0..=1).cycle() {
        let mut next = DiracMemo::new();
        for &roll in rolls.iter() {
            for (game, ngames) in games.iter() {
                let advanced = game.advance(p, roll);
                if advanced.players[p].score >= 21 {
                    wins[p] += ngames;
                } else {
                    *next.entry(advanced).or_default() += ngames;
                }
            }
        }
        games = next;
        if games.is_empty() {
            break;
        }
    }
    wins
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let p1_start: usize = lines
        .next()
        .map(|line| {
            line.unwrap()
                .split("Player 1 starting position: ")
                .skip(1)
                .next()
                .unwrap()
                .parse()
                .unwrap()
        })
        .unwrap();
    let p2_start: usize = lines
        .next()
        .map(|line| {
            line.unwrap()
                .split("Player 2 starting position: ")
                .skip(1)
                .next()
                .unwrap()
                .parse()
                .unwrap()
        })
        .unwrap();

    println!("Score: {}", practice(p1_start, p2_start));
    println!("Universes: {:?}", real(p1_start, p2_start));
}
