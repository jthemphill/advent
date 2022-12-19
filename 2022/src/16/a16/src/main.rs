use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};

const MAX_TIME: i64 = 30;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct RawNode {
    flow_rate: i64,
    tunnels: Vec<String>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    flow_rate: i64,
    edges: BTreeMap<String, i64>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Network {
    nodes: BTreeMap<String, Node>,
}

impl Network {
    fn from(raw_nodes: BTreeMap<String, RawNode>) -> Self {
        let mut nodes: BTreeMap<String, Node> = raw_nodes
            .into_iter()
            .map(|(node_key, raw_node)| {
                (
                    node_key,
                    Node {
                        flow_rate: raw_node.flow_rate,
                        edges: raw_node.tunnels.into_iter().map(|n| (n, 1)).collect(),
                    },
                )
            })
            .collect();

        let node_keys = nodes.keys().cloned().collect::<Vec<_>>();

        // Floyd-Warshall
        for via_key in &node_keys {
            for src_key in &node_keys {
                if let Some(src_to_via) = nodes.get(src_key).unwrap().edges.get(via_key).cloned() {
                    for dst_key in &node_keys {
                        if src_key != dst_key {
                            if let Some(via_to_dst) =
                                nodes.get(via_key).unwrap().edges.get(dst_key).cloned()
                            {
                                let detour = src_to_via + via_to_dst;
                                nodes
                                    .get_mut(src_key)
                                    .unwrap()
                                    .edges
                                    .entry(dst_key.clone())
                                    .and_modify(|src_to_dst| {
                                        *src_to_dst = (*src_to_dst).min(detour)
                                    })
                                    .or_insert(detour);
                            }
                        }
                    }
                }
            }
        }
        Self { nodes }
    }

    fn get(&self, node_key: &String) -> &Node {
        self.nodes.get(node_key).unwrap()
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct StateKey {
    pos: String,
    closed: BTreeSet<String>,
}

impl StateKey {
    fn new(pos: String, closed: BTreeSet<String>) -> Self {
        Self { pos, closed }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State {
    pos: String,
    open: BTreeMap<String, i64>,
    time: i64,
}

impl State {
    fn new() -> Self {
        Self {
            pos: "AA".to_string(),
            open: BTreeMap::new(),
            time: 0,
        }
    }

    fn key(&self, network: &Network) -> StateKey {
        StateKey::new(
            self.pos.clone(),
            network
                .nodes
                .keys()
                .filter(|n| !self.open.contains_key(*n))
                .cloned()
                .collect(),
        )
    }

    fn open_self(&self) -> Option<Self> {
        if self.open.contains_key(&self.pos) {
            None
        } else {
            let mut open = self.open.clone();
            open.insert(self.pos.clone(), self.time);
            Some(Self {
                pos: self.pos.clone(),
                open,
                time: self.time + 1,
            })
        }
    }

    fn enter(&self, node_key: String, dist: i64) -> Option<Self> {
        let time = self.time + dist;
        if time > MAX_TIME {
            None
        } else {
            Some(Self {
                pos: node_key,
                open: self.open.clone(),
                time,
            })
        }
    }

    fn score(&self, network: &Network) -> i64 {
        self.open
            .iter()
            .map(|(n, t)| network.get(n).flow_rate * (MAX_TIME - t - 1))
            .sum()
    }

    fn better_than_or_equal(&self, other: &State, network: &Network) -> bool {
        self.time <= other.time && self.score(network) >= other.score(network)
    }
}

#[derive(Debug)]
struct DFS {
    network: Network,
    seen: BTreeMap<StateKey, State>,
    states: Vec<State>,
}

impl DFS {
    fn new(raw_nodes: BTreeMap<String, RawNode>) -> Self {
        Self {
            network: Network::from(raw_nodes),
            seen: BTreeMap::new(),
            states: vec![State::new()],
        }
    }

    fn run(&mut self) -> Option<State> {
        let mut best: Option<State> = Option::None;
        while let Some(state) = self.states.pop() {
            let state_key = state.key(&self.network);
            if let Some(old_state) = self.seen.get(&state_key) {
                if old_state.better_than_or_equal(&state, &self.network) {
                    continue;
                }
            }
            self.seen.insert(state_key, state.clone());

            if let Some(old_best) = &best {
                if old_best.score(&self.network) < state.score(&self.network) {
                    println!("Best: {:?} {}", state, state.score(&self.network));
                    best = Some(state.clone());
                }
            } else {
                println!("Best: {:?} {}", state, state.score(&self.network));
                best = Some(state.clone());
            }

            let node = self.network.get(&state.pos);
            if node.flow_rate > 0 {
                if let Some(open_state) = state.open_self() {
                    self.states.push(open_state);
                }
            }
            for (dst_key, dist) in &node.edges {
                if self.network.get(dst_key).flow_rate > 0 {
                    if let Some(dst) = state.enter(dst_key.clone(), *dist) {
                        self.states.push(dst);
                    }
                }
            }
        }
        best
    }
}

fn main() {
    let re =
        Regex::new(r"Valve (.*) has flow rate=(\d*); tunnels? leads? to valves? (.*)").unwrap();
    let mut network = BTreeMap::new();
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            if let Some(caps) = re.captures(&line) {
                let valve = caps.get(1).unwrap().as_str().to_string();
                let flow_rate = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
                let tunnels = caps
                    .get(3)
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>();
                network.insert(valve.clone(), RawNode { flow_rate, tunnels });
            }
        }
    }
    let mut dfs = DFS::new(network);
    println!("{:?}", dfs);
    let best = dfs.run().unwrap();
    println!(
        "Best: {:?} ({}) (saw {} states)",
        best,
        best.score(&dfs.network),
        dfs.seen.len()
    );
}
