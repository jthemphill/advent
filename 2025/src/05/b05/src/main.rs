use std::{io::BufRead, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: i64,
    end: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRangeErr;

impl FromStr for Range {
    type Err = ParseRangeErr;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut parts = line.split('-');
        let start = parts
            .next()
            .ok_or(ParseRangeErr)?
            .parse()
            .map_err(|_| ParseRangeErr)?;
        let end = parts
            .next()
            .ok_or(ParseRangeErr)?
            .parse()
            .map_err(|_| ParseRangeErr)?;
        if end < start {
            Err(ParseRangeErr)
        } else {
            Ok(Self { start, end })
        }
    }
}

impl Range {
    fn overlaps(&self, other: &Range) -> bool {
        if other.start < self.start {
            other.overlaps(self)
        } else {
            other.start <= self.end
        }
    }

    fn merge(&self, other: &Range) -> Option<Range> {
        if self.overlaps(other) {
            Some(Range {
                start: self.start.min(other.start),
                end: self.end.max(other.end),
            })
        } else {
            None
        }
    }

    fn len(&self) -> i64 {
        self.end - self.start + 1
    }
}

fn main() {
    let mut ranges: Vec<Range> = Vec::new();
    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            if line.len() == 0 {
                break;
            } else {
                ranges.push(Range::from_str(&line).unwrap());
            }
        }
    }
    ranges.sort();

    let mut i = 0;
    let mut merged_ranges = Vec::with_capacity(ranges.len());
    while i < ranges.len() {
        let mut new_range = ranges[i];
        let mut j = i + 1;
        while j < ranges.len() {
            if let Some(merged_range) = new_range.merge(&ranges[j]) {
                new_range = merged_range;
            } else {
                break;
            }
            j += 1;
        }
        i = j;
        merged_ranges.push(new_range);
    }

    let mut num_fresh = 0;
    for range in &merged_ranges {
        num_fresh += range.len();
    }
    println!("{merged_ranges:?}");
    println!("{num_fresh} fresh");
}
