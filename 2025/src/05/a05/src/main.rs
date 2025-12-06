use std::io::BufRead;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn from(line: &str) -> Result<Self, <i64 as std::str::FromStr>::Err> {
        let mut parts = line.split('-');
        let start = parts.next().unwrap_or("").parse()?;
        let end = parts.next().unwrap_or("").parse()?;
        Ok(Self { start, end })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum InputMode {
    Range,
    Query,
}

fn query_in_ranges(ranges: &Vec<Range>, query: i64) -> bool {
    for range in ranges {
        if range.start <= query {
            if query <= range.end {
                return true;
            }
        } else {
            break;
        }
    }
    return false;
}

fn main() {
    let mut mode = InputMode::Range;
    let mut ranges: Vec<Range> = Vec::new();
    let mut queries: Vec<i64> = Vec::new();
    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            if line.len() == 0 {
                mode = InputMode::Query;
                continue;
            } else if mode == InputMode::Range {
                ranges.push(Range::from(&line).unwrap());
            } else {
                queries.push(line.parse().unwrap());
            }
        }
    }
    ranges.sort();

    let mut num_fresh = 0;
    for query in queries {
        if query_in_ranges(&ranges, query) {
            num_fresh += 1;
        }
    }
    println!("{num_fresh} fresh");
}
