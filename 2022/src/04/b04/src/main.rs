struct Range {
    start: u32,
    finish: u32,
}

impl Range {
    pub fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.finish && self.finish >= other.start
    }
}

impl From<&str> for Range {
    fn from(range: &str) -> Range {
        let mut range = range.split('-');
        let start = range.next().unwrap().parse().unwrap();
        let finish = range.next().unwrap().parse().unwrap();
        Range { start, finish }
    }
}

fn main() {
    let mut cnt = 0;
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let mut pair = line.split(',');
            let range1 = Range::from(pair.next().unwrap());
            let range2 = Range::from(pair.next().unwrap());
            if range1.overlaps(&range2) || range2.overlaps(&range1) {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
