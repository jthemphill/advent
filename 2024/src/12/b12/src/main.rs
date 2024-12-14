use std::{ascii::escape_default, collections::HashMap, io::Read};

type Regions = HashMap<(i32, i32), usize>;
type Areas = HashMap<usize, (u8, usize)>;
type Sides = HashMap<usize, usize>;

fn get_regions(grid: &Vec<u8>) -> (Regions, Areas, i32, i32) {
    let width = grid.iter().position(|&c| c == b'\n').unwrap() as i32;
    assert_ne!(width, 0);
    let height = grid.len() as i32 / width;

    let get = |x: i32, y: i32| {
        if 0 <= x && x < width && 0 <= y && y < height {
            Some(grid[(y * (width + 1) + x) as usize])
        } else {
            None
        }
    };

    let mut regions = Regions::with_capacity(height as usize * width as usize);
    let mut areas = Areas::new();
    for y in 0..height {
        for x in 0..width {
            if regions.contains_key(&(x, y)) {
                continue;
            }

            let label = get(x, y).unwrap();

            let mut area = 0;
            let mut stack: Vec<(i32, i32)> = vec![(x as i32, y as i32)];
            while let Some((x, y)) = stack.pop() {
                if regions.insert((x, y), areas.len()).is_some() {
                    continue;
                }

                area += 1;

                // Flood fill to neighbors
                for (dx, dy) in vec![(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    let nx = x + dx;
                    let ny = y + dy;
                    if get(nx, ny) == Some(label) {
                        stack.push((nx, ny));
                    }
                }
            }

            assert!(areas.insert(areas.len(), (label, area)).is_none());
        }
    }

    (regions, areas, width, height)
}

fn num_sides(regions: &Regions, width: i32, height: i32) -> Sides {
    let mut sides = Sides::new();

    for y in 0..height {
        for x in 0..width {
            let &region = regions.get(&(x, y)).unwrap();

            let is_in_region = |x: i32, y: i32| regions.get(&(x, y)) == Some(&region);

            // Look for corners. The number of corners always equals the number of sides.
            for (dx, dy) in vec![(-1, -1), (1, -1), (1, 1), (-1, 1)] {
                if is_in_region(x + dx, y + dy) {
                    // If the diagonally-adjacent square is in the region, both adjacent squares must be out of the region
                    if (!is_in_region(x + dx, y)) && (!is_in_region(x, y + dy)) {
                        *sides.entry(region).or_default() += 1;
                        println!(
                            "** Corner found for {} at {:2?} -> {:2?} because {:5?} {:5?}",
                            region,
                            (x, y),
                            (x + dx, y + dy),
                            (x + dx, y, is_in_region(x + dx, y),),
                            (x, y + dy, is_in_region(x, y + dy),),
                        );
                    }
                } else {
                    // If the diagonally adjacent square is not in the region,
                    // and if the two squares adjacent to the diagonal square are
                    // both in the region or both out of the region
                    if (is_in_region(x + dx, y)) == (is_in_region(x, y + dy)) {
                        *sides.entry(region).or_default() += 1;
                        println!(
                            "   Corner found for {} at {:2?} -> {:2?} because {:5?} {:5?}",
                            region,
                            (x, y),
                            (x + dx, y + dy),
                            (x + dx, y, is_in_region(x + dx, y),),
                            (x, y + dy, is_in_region(x, y + dy),),
                        );
                    }
                }
            }
        }
    }
    sides
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let input: Vec<u8> = input.as_bytes().to_vec();

    let mut total_score = 0;
    let (regions, areas, width, height) = get_regions(&input);
    let region_sides = num_sides(&regions, width, height);
    for region in 0..areas.len() {
        let sides = region_sides.get(&region).unwrap();
        let &(label, area) = areas.get(&region).unwrap();
        let score = area * sides;
        println!("{}: {area} * {sides} = {score}", escape_default(label));
        total_score += score;
    }

    println!("{total_score}");
}
