use std::{collections::HashMap, io::BufRead};

fn get_num_paths(
    cache: &mut HashMap<(String, bool, bool), usize>,
    adjs: &HashMap<String, Vec<String>>,
    (label1, mut dac, mut fft): (&str, bool, bool),
) -> usize {
    dac = dac || label1 == "dac";
    fft = fft || label1 == "fft";

    if let Some(ans) = cache.get(&(label1.to_owned(), dac, fft)) {
        return *ans;
    }
    if label1 == "out" {
        if dac && fft {
            return 1;
        } else {
            return 0;
        }
    }

    let mut total = 0;
    for label2 in adjs.get(label1).unwrap() {
        total += get_num_paths(cache, adjs, (label2, dac, fft));
    }
    cache.insert((label1.to_owned(), dac, fft), total);
    total
}

fn main() {
    let mut adjs: HashMap<String, Vec<String>> = HashMap::new();

    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            let mut parts = line.split_whitespace();
            let node1 = parts.next().unwrap().trim_end_matches(':');

            adjs.insert(
                node1.to_owned(),
                parts.into_iter().map(|s| s.to_owned()).collect(),
            );
        }
    }

    let mut cache = HashMap::new();
    let num_you_paths = get_num_paths(&mut cache, &adjs, ("svr", false, false));
    println!("Num paths: {num_you_paths}");
}
