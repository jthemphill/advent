use core::num;
use std::{
    ascii::escape_default,
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Binary,
    io::Read,
};

type Edge = (u8, u8);

const NUMERIC_KEYS: &[u8] = &[
    b'A', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
];

const DIRECTIONAL_KEYS: &[u8] = &[b'A', b'^', b'<', b'v', b'>'];

fn get_numeric_edges(pos: u8) -> &'static [Edge] {
    match pos {
        b'A' => &[(b'<', b'0'), (b'^', b'3')],
        b'0' => &[(b'>', b'A'), (b'^', b'2')],
        b'1' => &[(b'>', b'2'), (b'^', b'4')],
        b'2' => &[(b'v', b'0'), (b'<', b'1'), (b'>', b'3'), (b'^', b'5')],
        b'3' => &[(b'v', b'A'), (b'<', b'2'), (b'^', b'6')],
        b'4' => &[(b'v', b'1'), (b'>', b'5'), (b'^', b'7')],
        b'5' => &[(b'v', b'2'), (b'<', b'4'), (b'>', b'6'), (b'^', b'8')],
        b'6' => &[(b'v', b'3'), (b'<', b'5'), (b'^', b'9')],
        b'7' => &[(b'v', b'4'), (b'>', b'8')],
        b'8' => &[(b'v', b'5'), (b'<', b'7'), (b'>', b'9')],
        b'9' => &[(b'v', b'6'), (b'<', b'8')],
        _ => panic!("Unexpected position: {}", escape_default(pos)),
    }
}

fn get_direction_edges(pos: u8) -> &'static [Edge] {
    match pos {
        b'A' => &[(b'v', b'>'), (b'<', b'^')],
        b'^' => &[(b'v', b'v'), (b'>', b'A')],
        b'<' => &[(b'>', b'v')],
        b'v' => &[(b'<', b'<'), (b'>', b'>'), (b'^', b'^')],
        b'>' => &[(b'<', b'v'), (b'^', b'A')],
        _ => panic!("Unexpected position: {}", escape_default(pos)),
    }
}

fn apply_directional_input(
    edges_function: &dyn Fn(u8) -> &'static [Edge],
    position: u8,
    directional_input: u8,
) -> Option<u8> {
    edges_function(position)
        .iter()
        .find(|&&(direction, _)| direction == directional_input)
        .map(|(_, new_position)| *new_position)
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State {
    numeric_key: u8,
    directional_keys: [u8; 25],
}

impl State {
    fn input_to_directional_keypad(
        &self,
        layer: usize,
        directional_input: u8,
    ) -> Option<(Self, bool)> {
        match directional_input {
            b'A' => {
                if self.directional_keys.is_empty() {
                    self.input_to_numeric_keypad(directional_input)
                } else if layer == self.directional_keys.len() - 1 {
                    self.input_to_numeric_keypad(self.directional_keys[layer])
                } else {
                    self.input_to_directional_keypad(layer + 1, self.directional_keys[layer])
                }
            }
            _ => apply_directional_input(
                &get_direction_edges,
                self.directional_keys[layer],
                directional_input,
            )
            .map(|new_directional_key| {
                let mut next_state = *self;
                next_state.directional_keys[layer] = new_directional_key;
                (next_state, false)
            }),
        }
    }

    fn input_to_numeric_keypad(&self, directional_input: u8) -> Option<(Self, bool)> {
        match directional_input {
            b'A' => Some((*self, true)),
            _ => apply_directional_input(&get_numeric_edges, self.numeric_key, directional_input)
                .map(|numeric_key| {
                    let mut next_state = *self;
                    next_state.numeric_key = numeric_key;
                    (next_state, false)
                }),
        }
    }

    fn estimated_distance_to_number(&self, target_number: u8) -> usize {
        let mut estimate = 1;
        for &direction in &self.directional_keys {
            if direction != b'A' {
                estimate *= 2;
            }
        }
        if self.numeric_key != target_number {
            estimate *= 2;
        }
        estimate
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct AStarState {
    estimate: usize,
    cost: usize,
    num_output: usize,
    state: State,
}

impl Ord for AStarState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Lowest cost, then greatest output
        other
            .estimate
            .cmp(&self.estimate)
            .then(other.cost.cmp(&self.cost))
            .then(self.num_output.cmp(&other.num_output))
            .then(self.state.cmp(&other.state))
    }
}

impl PartialOrd for AStarState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn astar(start: State, code: &[u8]) -> usize {
    let mut queue = BinaryHeap::new();
    queue.push(AStarState {
        num_output: 0,
        cost: 0,
        estimate: start.estimated_distance_to_number(code[0]) + code.len() - 1,
        state: start.clone(),
    });

    let mut history = HashMap::new();
    while let Some(astar_state) = queue.pop() {
        let AStarState {
            num_output,
            cost,
            estimate,
            state,
        } = astar_state;
        if num_output == code.len() {
            return cost;
        }

        let mut directional_inputs = vec![b'A'];
        for &(direction, _) in get_direction_edges(state.directional_keys[0]) {
            directional_inputs.push(direction);
        }
        for directional_input in directional_inputs {
            if let Some((next_state, did_output_code)) =
                state.input_to_directional_keypad(0, directional_input)
            {
                if did_output_code && state.numeric_key != code[num_output] {
                    continue;
                }
                let next_num_output = num_output + if did_output_code { 1 } else { 0 };

                match history.entry((next_num_output, next_state)) {
                    Entry::Occupied(_) => {
                        continue;
                    }
                    Entry::Vacant(entry) => {
                        entry.insert((directional_input, (num_output, state)));
                    }
                }

                // println!(
                //     "{}: {} -> {} -> {}",
                //     next_num_output,
                //     escape_default(state.directional_keys[0]),
                //     escape_default(directional_input),
                //     escape_default(next_state.directional_keys[0])
                // );
                let next_estimate = if next_num_output < code.len() {
                    cost + 1 + state.estimated_distance_to_number(code[next_num_output])
                } else {
                    0
                };
                queue.push(AStarState {
                    cost: cost + 1,
                    estimate: next_estimate,
                    num_output: next_num_output,
                    state: next_state,
                });
            }
        }
    }
    panic!("Couldn't input code {}", String::from_utf8_lossy(code));
}

fn pathfind(start: u8, target: u8, edges_function: &dyn Fn(u8) -> &'static [Edge]) -> Vec<u8> {
    let mut history = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back((0, start));

    let mut found = false;
    while let Some((cost, pos)) = queue.pop_front() {
        if pos == target {
            found = true;
            break;
        }
        for &(dir, next) in edges_function(pos) {
            let entry = history.entry(next);
            match entry {
                Entry::Occupied(_) => {
                    continue;
                }
                Entry::Vacant(entry) => {
                    entry.insert((dir, pos));
                    queue.push_back((cost + 1, next));
                }
            }
        }
    }
    if !found {
        panic!(
            "Couldn't reach {} from {}",
            escape_default(target),
            escape_default(start)
        );
    }

    let mut path = vec![];
    let mut pos = target;
    while pos != start {
        let (dir, before) = *history.get(&pos).unwrap();
        path.push(dir);
        pos = before;
    }

    path.reverse();
    path
}

fn input_code(code: &[u8], start: u8, edges_function: &dyn Fn(u8) -> &'static [Edge]) -> Vec<u8> {
    let mut pos = start;
    let mut dirs = vec![];
    for &target in code {
        for dir in pathfind(pos, target, edges_function) {
            dirs.push(dir);
        }
        dirs.push(b'A');
        pos = target;
    }
    dirs
}

fn code_to_number(code: Vec<u8>) -> usize {
    let mut num = 0;

    for c in code {
        if c.is_ascii_digit() {
            num *= 10;
            num += (c - b'0') as usize;
        }
    }

    num
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let codes: Vec<Vec<u8>> = input
        .split('\n')
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let mut total_complexity = 0;
    for code in codes {
        let cost = astar(
            State {
                numeric_key: b'A',
                directional_keys: [b'A'; 25],
            },
            &code,
        );
        let num = code_to_number(code);
        let complexity = cost * num;
        println!("{} * {} = {}", cost, num, complexity);
        total_complexity += complexity;
    }
    println!("Total: {total_complexity}");

    // let numeric_code = "029A".as_bytes();
    // let direction_code1 = input_code(&numeric_code, b'A', &get_numeric_edges);
    // let direction_code2 = input_code(&direction_code1, b'A', &get_direction_edges);
    // let direction_code3 = input_code(&direction_code2, b'A', &get_direction_edges);

    // println!("{}", String::from_utf8_lossy(&direction_code3));
}

// fn bad_floyd_warshall() {

//     let mut states_to_ids = HashMap::new();
//     for &numeric_key in NUMERIC_KEYS {
//         for &directional_key1 in DIRECTIONAL_KEYS {
//             for &directional_key2 in DIRECTIONAL_KEYS {
//                 let state = State {
//                     numeric_key,
//                     directional_keys: vec![directional_key1, directional_key2],
//                 };
//                 states_to_ids.insert(state, states_to_ids.len());
//             }
//         }
//     }
//     let mut distance = vec![vec![std::usize::MAX; states_to_ids.len()]; states_to_ids.len()];
//     for state in states_to_ids.keys() {
//         for &directional_input in DIRECTIONAL_KEYS {
//             let mut (next_state, _) =
//                 state.input_to_directional_keypad(state.directional_keys.len(), directional_input);

//             let &id = states_to_ids.get(state).unwrap();
//             let &next_id = states_to_ids.get(next_state).unwrap();
//             distance[id][next_id] = 1;
//         }
//     }
// }
