use once_cell::sync::Lazy;
use std::collections::HashMap;

const ORIGIN_X: i64 = 2;
const ORIGIN_Y: i64 = 3;
const WIDTH: i64 = 7;
const NUM_SHAPES: usize = 5;

type Pos = (i64, i64);
type Shape = Vec<&'static [u8]>;
type Grid = HashMap<Pos, bool>;

#[rustfmt::skip]
static SHAPES: Lazy<[Shape; NUM_SHAPES]> = Lazy::new(|| [
    vec![
        b"####",
    ],
    vec![
        b".#.",
        b"###",
        b".#.",
    ],
    vec![
        b"..#",
        b"..#",
        b"###",
    ],
    vec![
        b"#",
        b"#",
        b"#",
        b"#",
    ],
    vec![
        b"##",
        b"##",
    ],
]);

fn check(grid: &Grid, pos: Pos, shape: &Shape) -> bool {
    let (x, y) = pos;
    for (dy, row) in shape.into_iter().rev().enumerate() {
        let y = y + dy as i64;
        if y < 0 {
            return false;
        }
        for (dx, c) in row.into_iter().enumerate() {
            let x = x + dx as i64;
            if x < 0 || x >= WIDTH {
                return false;
            }
            match c {
                b'#' => {
                    if grid
                        .get(&(x, y))
                        .cloned()
                        .unwrap_or(false)
                    {
                        return false;
                    }
                }
                b'.' => {}
                _ => panic!("Saw unexpected char `{}`", c),
            }
        }
    }
    true
}

fn subtract(grid: &mut Grid, pos: Pos, shape: &Shape) {
    let (x, y) = pos;
    for (dy, row) in shape.into_iter().rev().enumerate() {
        for (dx, c) in row.into_iter().enumerate() {
            match c {
                b'#' => {
                    let c = grid.entry((x + dx as i64, y + dy as i64)).or_insert(false);
                    *c = false;
                }
                b'.' => {}
                _ => panic!("Saw unexpected char `{}`", c),
            }
        }
    }
}

fn add(grid: &mut Grid, pos: Pos, shape: &Shape) {
    let (x, y) = pos;
    for (dy, row) in shape.into_iter().rev().enumerate() {
        let y = y + dy as i64;
        assert!(y >= 0);
        for (dx, c) in row.into_iter().enumerate() {
            let x = x + dx as i64;
            assert!(x >= 0);
            assert!(x < WIDTH);
            match c {
                b'#' => {
                    let c = grid.entry((x, y)).or_insert(false);
                    assert_eq!(*c, false);
                    *c = true;
                }
                b'.' => {}
                _ => panic!("Saw unexpected char `{}`", c),
            }
        }
    }
}

fn render(grid: &Grid, height: i64) {
    for y in (0..height + ORIGIN_Y + 4).rev() {
        let mut row = String::new();
        for x in 0..7 {
            row.push(if grid.get(&(x, y)).cloned().unwrap_or(false) {
                '#'
            } else {
                '.'
            });
        }
        println!("{}{}#", if y == height { 'h' } else { '#' }, row);
    }
    println!("#########");
}

fn main() {
    let mut pattern = vec![];
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            pattern.extend(line.bytes());
        }
    }
    let mut grid = HashMap::new();
    let mut height = 0;
    let mut num_dropped_shapes = 0;
    let mut pattern_i = 0;
    let mut x = ORIGIN_X;
    let mut y = height + ORIGIN_Y;
    let mut new_shape;
    add(&mut grid, (x, y), &SHAPES[num_dropped_shapes]);
    while num_dropped_shapes < 2022 {
        new_shape = false;
        // render(&grid, height);
        let shape = &SHAPES[num_dropped_shapes % NUM_SHAPES];
        let dx: i64 = match pattern[pattern_i % pattern.len()] {
            b'<' => -1,
            b'>' => 1,
            _ => panic!(
                "Unexpected pattern symbol: {}",
                pattern[pattern_i % pattern.len()]
            ),
        };
        subtract(&mut grid, (x, y), shape);
        if check(&grid, (x + dx, y), shape) {
            x += dx;
        }
        if check(&grid, (x, y - 1), shape) {
            y -= 1;
        } else {
            new_shape = true;
        }
        add(&mut grid, (x, y), shape);
        if new_shape {
            height = height.max(y + shape.len() as i64);
            num_dropped_shapes += 1;
            x = ORIGIN_X;
            y = height + ORIGIN_Y;
            let shape = &SHAPES[num_dropped_shapes % NUM_SHAPES];
            add(&mut grid, (x, y), shape);
            // render(&grid, height);
            // println!("");
        }
        pattern_i += 1;
    }
    println!("Final height: {}", height);
}
