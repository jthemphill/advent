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

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pattern {
    shape_i: usize,
    pattern_i: usize,
}

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
                    if grid.get(&(x, y)).cloned().unwrap_or(false) {
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
                    let c = grid.get_mut(&(x + dx as i64, y + dy as i64)).unwrap();
                    assert_eq!(*c, true);
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

// fn render_row(grid: &Grid, height: i64, y: i64) -> String {
//     let mut row = format!("{: >4} ", y);
//     row.push(if y == height { 'h' } else { '#' });
//     for x in 0..7 {
//         row.push(if grid.get(&(x, y)).cloned().unwrap_or(false) {
//             '#'
//         } else {
//             '.'
//         });
//     }
//     row.push('#');
//     row
// }

// fn render(grid: &Grid, height: i64) {
//     for y in (0..height + ORIGIN_Y + 4).rev() {
//         println!("{}", render_row(grid, height, y));
//     }
//     println!("     #########");
// }

fn find_cycle_len(seen: &Vec<(usize, i64)>) -> Option<(usize, i64)> {
    if seen.len() < 2 {
        None
    } else {
        let a = seen[seen.len() - 2];
        let b = seen[seen.len() - 1];
        Some(((b.0 - a.0), (b.1 - a.1)))
    }
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
    let mut num_instrs = 0;
    let mut x = ORIGIN_X;
    let mut y = height + ORIGIN_Y;
    let mut new_shape = false;

    let mut history: HashMap<Pattern, Vec<(usize, i64)>> = HashMap::new();
    let target_shapes = 1000000000000;

    add(&mut grid, (x, y), &SHAPES[num_dropped_shapes]);
    // render(&grid, height + ORIGIN_Y);
    loop {
        let shape_i = num_dropped_shapes % NUM_SHAPES;
        let pattern_i = num_instrs % pattern.len();
        if new_shape {
            let seen = history
                .entry(Pattern {
                    pattern_i,
                    shape_i: num_dropped_shapes % NUM_SHAPES,
                })
                .or_default();
            seen.push((num_dropped_shapes, height));
            if let Some((dshapes, dh)) = find_cycle_len(seen) {
                if (target_shapes - num_dropped_shapes) % dshapes == 0 {
                    let num_reps = (target_shapes - num_dropped_shapes) / dshapes;
                    println!(
                        "After {} repetitions, we'll be {} units tall",
                        num_reps,
                        height + num_reps as i64 * dh,
                    );
                    return;
                }
            }
        }
        new_shape = false;
        let shape = &SHAPES[shape_i];
        let dx: i64 = match pattern[pattern_i] {
            b'<' => -1,
            b'>' => 1,
            _ => panic!("Unexpected pattern symbol: {}", pattern[pattern_i]),
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
            let shape_i = num_dropped_shapes % NUM_SHAPES;
            x = ORIGIN_X;
            y = height + ORIGIN_Y;
            let shape = &SHAPES[shape_i];
            add(&mut grid, (x, y), shape);
        }
        num_instrs += 1;
    }
}
