use std::{collections::HashSet, io::Read};

type Regions = Vec<(u8, usize, usize)>;

fn regions(grid: &Vec<u8>) -> Regions {
    let width = grid.iter().position(|&c| c == b'\n').unwrap();
    assert_ne!(width, 0);
    let height = grid.len() / width;

    let get = |x: usize, y: usize| grid[y * (width + 1) + x];

    let mut seen: HashSet<usize> = HashSet::new();

    let mut regions = Regions::new();
    for y in 0..height {
        for x in 0..width {
            let offset = y * (width + 1) + x;
            if seen.contains(&offset) {
                continue;
            }

            let label = get(x, y);
            let mut area = 0;
            let mut perimeter = 0;

            let mut stack: Vec<(i32, i32)> = vec![(x as i32, y as i32)];
            while let Some((x, y)) = stack.pop() {
                let offset = y as usize * (width + 1) + x as usize;
                if !seen.insert(offset) {
                    continue;
                }

                area += 1;

                for (dx, dy) in vec![(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0
                        || nx as usize >= width
                        || ny < 0
                        || ny as usize >= height
                        || get(nx as usize, ny as usize) != label
                    {
                        perimeter += 1;
                    } else {
                        stack.push((nx, ny));
                    }
                }
            }
            regions.push((label, area, perimeter));
        }
    }

    regions
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let input: Vec<u8> = input.as_bytes().to_vec();

    let mut total_score = 0;
    for (region, area, perimeter) in regions(&input) {
        let score = area * perimeter;
        println!(
            "{}: {area} * {perimeter} = {score}",
            String::from_utf8(vec![region]).unwrap(),
        );
        total_score += score;
    }

    println!("{total_score}");
}
