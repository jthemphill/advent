use std::collections::HashSet;
use std::io::prelude::*;

type Points = HashSet<(i32, i32)>;

#[derive(Clone, Copy, Debug)]
enum Axis {
    X,
    Y,
}

impl From<&str> for Axis {
    fn from(s: &str) -> Axis {
        match s {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => panic!("Expected an axis, got {}", s),
        }
    }
}

fn print_grid(points: &Points) {
    let min_x = *points.iter().map(|(x, _)| x).min().unwrap();
    let max_x = *points.iter().map(|(x, _)| x).max().unwrap();

    let min_y = *points.iter().map(|(_, y)| y).min().unwrap();
    let max_y = *points.iter().map(|(_, y)| y).max().unwrap();
    for y in min_y..=max_y {
        let mut row = String::with_capacity((max_x - min_x) as usize);
        for x in min_x..=max_x {
            row.push(if points.contains(&(x, y)) { '#' } else { '.' });
        }
        println!("{}", row);
    }
    println!("{} dots visible.", points.len());
}

fn run_fold(points: &mut Points, (axis, line): (Axis, i32)) {
    let min_x = *points.iter().map(|(x, _)| x).min().unwrap();
    let max_x = *points.iter().map(|(x, _)| x).max().unwrap();
    let min_y = *points.iter().map(|(_, y)| y).min().unwrap();
    let max_y = *points.iter().map(|(_, y)| y).max().unwrap();

    let mut new_points = Points::new();
    match axis {
        Axis::X => {
            for x in min_x..line {
                for y in min_y..=max_y {
                    if points.contains(&(x, y)) {
                        new_points.insert((x, y));
                    }
                }
            }
            for x in line..=max_x {
                for y in min_y..=max_y {
                    if points.contains(&(x, y)) {
                        let folded_x = 2 * line - x;
                        new_points.insert((folded_x, y));
                    }
                }
            }
        }
        Axis::Y => {
            for x in min_x..=max_x {
                for y in min_y..line {
                    if points.contains(&(x, y)) {
                        new_points.insert((x, y));
                    }
                }
            }
            for x in min_x..=max_x {
                for y in line..=max_y {
                    if points.contains(&(x, y)) {
                        let folded_y = 2 * line - y;
                        new_points.insert((x, folded_y));
                    }
                }
            }
        }
    }
    std::mem::swap(points, &mut new_points);
}

fn run_folds(points: &mut Points, folds: &Vec<(Axis, i32)>) {
    print_grid(points);
    for fold in folds {
        println!("Folding {:?}", fold);
        run_fold(points, *fold);
        print_grid(points);
    }
}

fn main() {
    let mut points = Points::new();
    let mut add_points = true;
    let mut folds: Vec<(Axis, i32)> = vec![];
    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            println!("Line: {}", line);
            if add_points {
                if line.is_empty() {
                    add_points = false;
                    continue;
                }

                let mut parts = line.split(",");
                points.insert((
                    parts.next().unwrap().parse().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                ));
            } else {
                let mut fold_parts = line.split("fold along ");
                fold_parts.next();
                let mut fold_parts = fold_parts.next().unwrap().split('=');
                folds.push((
                    Axis::from(fold_parts.next().unwrap()),
                    fold_parts.next().unwrap().parse().unwrap(),
                ))
            }
        }
    }

    run_folds(&mut points, &folds);
}
