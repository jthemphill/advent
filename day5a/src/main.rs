fn parse_pair(token: &str) -> (usize, usize) {
    let mut nums = token.split(",");
    (
        nums.next().unwrap().parse::<usize>().unwrap(),
        nums.next().unwrap().parse::<usize>().unwrap(),
    )
}

fn simplify(mut x: usize, mut y: usize) -> (usize, usize) {
    println!("Simplifying {}, {}", x, y);
    for factor in 2..=x.min(y) {
        while x % factor == 0 && y % factor == 0 {
            x /= factor;
            y /= factor;
        }
    }
    (x, y)
}

fn main() {
    let mut lines = vec![];
    for line in include_str!("../input.txt").lines() {
        let mut tokens = line.split(" ");
        let left = parse_pair(tokens.next().unwrap());
        assert_eq!(tokens.next().unwrap(), "->");
        let right = parse_pair(tokens.next().unwrap());
        lines.push((left, right));
    }

    let xmax = *lines
        .iter()
        .map(|((x1, _), (x2, _))| x1.max(x2))
        .max()
        .unwrap();
    let ymax = *lines
        .iter()
        .map(|((_, y1), (_, y2))| y1.max(y2))
        .max()
        .unwrap();
    let mut grid = vec![vec![0; ymax + 1]; xmax + 1];
    for ((mut x1, mut y1), (mut x2, mut y2)) in lines {
        if x2 < x1 {
            std::mem::swap(&mut x1, &mut x2);
            std::mem::swap(&mut y1, &mut y2);
        }
        let (dx, dy) = simplify(x2.max(x1) - x2.min(x1), y2.max(y1) - y2.min(y1));
        println!("{}, {}", dx, dy);
        if dx == 0 {
            for y in y1.min(y2)..=y1.max(y2) {
                grid[x1][y] += 1;
            }
        } else if dy == 0 {
            for x in x1.min(x2)..=x1.max(x2) {
                grid[x][y1] += 1;
            }
        } else if y1 < y2 {
            let mut x = x1.min(x2);
            let mut y = y1.min(y2);
            while x <= x1.max(x2) && y <= y1.max(y2) {
                grid[x][y] += 1;
                x += dx;
                y += dy;
            }
        } else {
            let (mut x, mut y) = (x1, y1);
            while x <= x2 && y >= y2 {
                println!("{}, {}", x, y);
                grid[x][y] += 1;
                x += dx;
                if y < dy {
                    break;
                }
                y -= dy;
            }
        }
    }
    let mut rendered = String::new();
    for y in 0..=ymax {
        for x in 0..=xmax {
            if grid[x][y] == 0 {
                rendered.push('.');
            } else {
                rendered.push_str(&grid[x][y].to_string());
            }
        }
        rendered.push('\n');
    }
    println!("{}", rendered);
    let mut intersections = 0;
    for x in 0..=xmax {
        for y in 0..=ymax {
            if grid[x][y] > 1 {
                intersections += 1;
            }
        }
    }
    println!("{} intersections", intersections);
}
