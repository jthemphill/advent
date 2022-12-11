use std::iter;

const W: usize = 40;

struct CRT {
    rows: Vec<String>,
    t: usize,
    x: i32,
}

impl CRT {
    fn new() -> Self {
        CRT {
            rows: vec![String::new()],
            t: 1,
            x: 1,
        }
    }

    fn noop(&mut self) {
        println!("\nStart cycle {}: begin executing noop", self.t);
        self.cycle();
        println!("End of cycle {}: finish executing noop", self.t - 1);
    }

    fn addx(&mut self, dx: i32) {
        println!("\nStart cycle {}: begin executing addx {}", self.t, dx);
        self.cycle();
        println!();
        self.cycle();
        self.x += dx;
        println!(
            "End of cycle {}: finish executing addx {} (Register X is now {})",
            self.t - 1,
            dx,
            self.x
        );
        self.print_sprite();
    }

    fn cycle(&mut self) {
        let cur_row = self.rows.last_mut().unwrap();

        let pos = cur_row.len();
        println!(
            "During cycle {}: CRT draws pixel in position {}",
            self.t, pos
        );
        cur_row.push(if self.x - 1 <= pos as i32 && pos as i32 <= self.x + 1 {
            '#'
        } else {
            '.'
        });
        println!("Current CRT row: {}", cur_row);
        if cur_row.len() == W {
            self.rows.push(String::new());
        }

        self.t += 1;
    }

    fn print_sprite(&self) {
        let x = self.x;
        println!(
            "Sprite position: {}{}{}",
            iter::repeat('.')
                .take((x - 1).clamp(0, W as i32) as usize)
                .collect::<String>(),
            if x == -1 || x == W as i32 + 1 {
                "#"
            } else if x == 0 || x == W as i32 {
                "##"
            } else if 1 <= x && x <= W as i32 - 1 {
                "###"
            } else {
                ""
            },
            iter::repeat('.')
                .take((W as i32 + 1 - 3 - x) as usize)
                .collect::<String>()
        );
    }

    fn display(&self) {
        for row in &self.rows {
            println!("{}", row);
        }
    }
}

fn main() {
    let mut crt = CRT::new();

    crt.print_sprite();
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let mut split = line.split(" ");
            let cmd = split.next().unwrap();
            match cmd {
                "noop" => crt.noop(),
                "addx" => crt.addx(split.next().unwrap().parse::<i32>().unwrap()),
                _ => panic!("Unrecognized command: {}", cmd),
            };
        }
    }

    println!("Final output:");
    crt.display();
}
