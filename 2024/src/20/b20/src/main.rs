use std::{
    ascii::escape_default,
    collections::{hash_map::Entry, HashMap},
    io::Read,
};

type Pos = (i32, i32);

struct Grid {
    data: Vec<u8>,
    width: i32,
    height: i32,
    start: Pos,
    end: Pos,
}

impl Grid {
    fn new(data: Vec<u8>) -> Self {
        let width = data.iter().position(|&c| c == b'\n').unwrap() as i32;
        let height = data.len() as i32 / (width + 1);
        let mut start = None;
        let mut end = None;
        for y in 0..height {
            for x in 0..width {
                match data[(y * (width + 1) + x) as usize] {
                    b'S' => {
                        start = Some((x, y));
                    }
                    b'E' => {
                        end = Some((x, y));
                    }
                    b'#' | b'.' => {}
                    c => {
                        panic!(
                            "Unexpected character at {:?}: {}",
                            (x, y),
                            escape_default(c)
                        )
                    }
                }
            }
        }
        Self {
            data,
            width,
            height,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }

    fn get(&self, (x, y): Pos) -> u8 {
        if !(0 <= x && x < self.width && 0 <= y && y < self.height) {
            return b'#';
        }

        self.data[(y * (self.width + 1) + x) as usize]
    }
}

fn get_times(grid: &Grid) -> (HashMap<Pos, usize>, HashMap<(Pos, Pos), usize>) {
    let mut time_from_pos_to_end = HashMap::new();
    time_from_pos_to_end.insert(grid.end, 0);
    let mut time_to_end = 0;
    let mut pos = grid.end;

    let mut cheats = HashMap::new();
    loop {
        let (x, y) = pos;
        for dx in -20..=20_i32 {
            for dy in -20..=20_i32 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let cheat_distance = (dx.abs() + dy.abs()) as usize;
                if cheat_distance > 20 {
                    continue;
                }
                let (x2, y2) = (x + dx, y + dy);
                if let Some(time_after_cheat) = time_from_pos_to_end.get(&(x2, y2)) {
                    let time_saved = time_to_end - cheat_distance - time_after_cheat;
                    cheats.insert((pos, (x2, y2)), time_saved);
                }
            }
        }

        if pos == grid.start {
            break;
        }
        time_to_end += 1;
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (x2, y2) = (x + dx, y + dy);
            match grid.get((x2, y2)) {
                b'E' | b'#' => {}
                b'.' | b'S' => {
                    match time_from_pos_to_end.entry((x2, y2)) {
                        Entry::Occupied(_) => {
                            continue;
                        }
                        Entry::Vacant(e) => e.insert(time_to_end),
                    };
                    pos = (x2, y2);
                    break;
                }
                c => panic!("Unexpected char: {}", escape_default(c)),
            }
        }
    }
    (time_from_pos_to_end, cheats)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let grid = Grid::new(input.as_bytes().to_vec());

    let (fair_times, cheats) = get_times(&grid);
    let fair_time = fair_times.get(&grid.start).unwrap();
    println!("Fair time: {fair_time}");

    let mut cheats_by_time_saved: HashMap<usize, usize> = HashMap::new();
    for (_, time_saved) in cheats {
        *cheats_by_time_saved.entry(time_saved).or_default() += 1;
    }

    let mut times: Vec<usize> = cheats_by_time_saved.keys().cloned().collect();
    times.sort();
    let mut num_good_cheats = 0;
    for time_saved in &times {
        // println!(
        //     "There are {} cheats that save {}",
        //     cheats_by_time_saved.get(time_saved).unwrap(),
        //     time_saved
        // );
        if *time_saved >= 100 {
            num_good_cheats += cheats_by_time_saved.get(time_saved).unwrap();
        }
    }
    println!("{num_good_cheats} good cheats"); // 376 is too low
}
