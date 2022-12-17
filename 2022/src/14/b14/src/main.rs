use std::collections::HashMap;

#[derive(Default, Eq, PartialEq)]
enum Cell {
    #[default]
    Air,
    Rock,
    Sand,
}

fn main() {
    let mut grid = HashMap::new();
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let mut last_point: Option<(i32, i32)> = Option::None;
            for point in line.split(" -> ") {
                let mut p = point.split(",");
                let new_x = p.next().unwrap().parse::<i32>().unwrap();
                let new_y = p.next().unwrap().parse::<i32>().unwrap();
                if let Some((last_x, last_y)) = last_point {
                    if new_x == last_x {
                        for y in last_y.min(new_y)..=last_y.max(new_y) {
                            grid.insert((last_x, y), Cell::Rock);
                        }
                    } else if new_y == last_y {
                        for x in last_x.min(new_x)..=last_x.max(new_x) {
                            grid.insert((x, last_y), Cell::Rock);
                        }
                    } else {
                        panic!(
                            "({}, {}) and ({}, {}) do not share a row or column",
                            last_x, last_y, new_x, new_y
                        );
                    }
                }
                last_point = Some((new_x, new_y));
            }
        }
    }

    let min_x = *grid.keys().map(|(x, _y)| x).min().unwrap();
    let max_x = *grid.keys().map(|(x, _y)| x).max().unwrap();
    let max_y = *grid.keys().map(|(_x, y)| y).max().unwrap();
    println!("Dimensions: {}, {}", max_x, max_y);

    let mut num_sand = 0;
    'outer: loop {
        let mut x = 500;
        let mut y = 0;
        'inner: loop {
            if y == max_y + 1 {
                grid.insert((x, y), Cell::Sand);
                break 'inner;
            } else if *grid.get(&(x, y + 1)).unwrap_or(&Cell::Air) == Cell::Air {
                y += 1;
            } else if *grid.get(&(x - 1, y + 1)).unwrap_or(&Cell::Air) == Cell::Air {
                x -= 1;
                y += 1;
            } else if *grid.get(&(x + 1, y + 1)).unwrap_or(&Cell::Air) == Cell::Air {
                x += 1;
                y += 1;
            } else if x == 500 && y == 0 {
                num_sand += 1;
                break 'outer;
            } else {
                grid.insert((x, y), Cell::Sand);
                break 'inner;
            }
        }
        // for y in 0..=max_y {
        //     let mut row = String::new();
        //     for x in min_x..=max_x {
        //         row.push(match *grid.get(&(x, y)).unwrap_or(&Cell::Air) {
        //             Cell::Air => '.',
        //             Cell::Rock => '#',
        //             Cell::Sand => 'o',
        //         });
        //     }
        //     println!("{}", row);
        // }
        num_sand += 1;
    }
    println!("{}", num_sand);
}
