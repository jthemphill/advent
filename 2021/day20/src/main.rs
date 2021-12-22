use std::io::prelude::*;

type Image = std::collections::HashMap<(i64, i64), bool>;

struct Input {
    alg: Vec<bool>,
    img: Image,
    bg: bool,
}

impl Input {
    fn read() -> Self {
        let stdin = std::io::stdin();
        let mut lines = stdin.lock().lines().map(|line| line.unwrap());

        let alg: Vec<bool> = lines.next().unwrap().chars().map(|c| c == '#').collect();
        assert_eq!(alg.len(), 512);

        assert!(lines.next().unwrap().is_empty());

        let mut img: Image = Image::new();
        for (i, line) in lines.enumerate() {
            for (j, c) in line.chars().enumerate() {
                img.insert((i as i64, j as i64), c == '#');
            }
        }

        Self {
            alg,
            img,
            bg: false,
        }
    }

    fn get(&self, i: i64, j: i64) -> bool {
        *self.img.get(&(i, j)).unwrap_or(&self.bg)
    }

    fn i_dims(&self) -> (i64, i64) {
        let min_i = self.img.iter().map(|(&(i, _), &_)| i).min().unwrap();
        let max_i = self.img.iter().map(|(&(i, _), &_)| i).max().unwrap();
        (min_i, max_i)
    }

    fn j_dims(&self) -> (i64, i64) {
        let min_j = self.img.iter().map(|(&(_, j), &_)| j).min().unwrap();
        let max_j = self.img.iter().map(|(&(_, j), &_)| j).max().unwrap();
        (min_j, max_j)
    }

    fn decode(self) -> Input {
        let (min_i, max_i) = self.i_dims();
        let (min_j, max_j) = self.j_dims();

        let mut img: Image = Image::new();
        for i in min_i - 1..=max_i + 1 {
            for j in min_j - 1..=max_j + 1 {
                let mut idx = 0;
                for di in -1..=1 {
                    for dj in -1..=1 {
                        idx *= 2;
                        let lit = *self.img.get(&(i + di, j + dj)).unwrap_or(&self.bg);
                        idx += if lit { 1 } else { 0 };
                    }
                }
                let c = self.alg[idx];
                img.insert((i, j), c);
            }
        }
        let bg = if self.bg {
            self.alg[1 << 9 - 1]
        } else {
            self.alg[0]
        };

        Input {
            alg: self.alg,
            img,
            bg,
        }
    }

    fn num_lit(&self) -> Option<usize> {
        if self.bg {
            None
        } else {
            Some(self.img.iter().filter(|(&_, &lit)| lit).count())
        }
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_i, max_i) = self.i_dims();
        let (min_j, max_j) = self.j_dims();

        let mut display = String::new();
        for i in min_i - 5..max_i + 5 {
            for j in min_j - 5..max_j + 5 {
                display.push(if self.get(i, j) { '#' } else { '.' });
            }
            display.push('\n');
        }
        write!(f, "{}", display)
    }
}

fn main() {
    let mut input = Input::read();
    println!("{}", input);

    for _ in 0..2 {
        input = input.decode();
    }
    println!("{} pixels lit after 2 iterations", input.num_lit().unwrap());

    for _ in 2..50 {
        input = input.decode();
    }
    println!("{}", input);
    println!("{} pixels lit after 50 iterations", input.num_lit().unwrap());
}
