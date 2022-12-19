use regex::Regex;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

type Label = [u8; 2];

const MAX_TIME: i64 = 26;
const START: Label = [b'A', b'A'];
const ITERS: u8 = 2;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct RawNode {
    flow_rate: i64,
    tunnels: Vec<Label>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    flow_rate: i64,
    edges: BTreeMap<Label, i64>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Network {
    nodes: BTreeMap<Label, Node>,
}

impl Network {
    fn from(raw_nodes: BTreeMap<Label, RawNode>) -> Self {
        let mut nodes: BTreeMap<Label, Node> = raw_nodes
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

    fn get(&self, node_key: &Label) -> &Node {
        self.nodes.get(node_key).unwrap()
    }

    fn distance(&self, node1: &Label, node2: &Label) -> i64 {
        if node1 == node2 {
            0
        } else {
            *self.get(node1).edges.get(node2).unwrap()
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct StateKey {
    pos: Label,
    closed: BTreeSet<Label>,
}

impl StateKey {
    fn new(pos: Label, closed: BTreeSet<Label>) -> Self {
        Self { pos, closed }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State {
    pos: Label,
    open: BTreeMap<Label, i64>,
    time: i64,
    iter: u8,
}

impl State {
    fn new() -> Self {
        Self {
            pos: START,
            open: BTreeMap::new(),
            time: 0,
            iter: 1,
        }
    }

    fn adjacent_states(&self, network: &Network) -> Vec<Self> {
        let mut new_states = vec![];
        if self.time < MAX_TIME && !self.open.contains_key(&self.pos) {
            let mut open = self.open.clone();
            open.insert(self.pos, self.time + 1);
            new_states.push(Self {
                pos: self.pos,
                open,
                time: self.time + 1,
                iter: self.iter,
            })
        }
        for (dst, dist) in &network.get(&self.pos).edges {
            let time = self.time + dist;
            if time < MAX_TIME {
                if network.get(&dst).flow_rate > 0 {
                    new_states.push(Self {
                        pos: *dst,
                        open: self.open.clone(),
                        time,
                        iter: self.iter,
                    });
                }
            }
        }
        if self.iter < ITERS {
            new_states.push(Self {
                pos: START,
                open: self.open.clone(),
                time: 0,
                iter: self.iter + 1,
            });
        }
        new_states
    }

    fn key(&self, network: &Network) -> StateKey {
        StateKey::new(
            self.pos,
            network
                .nodes
                .keys()
                .filter(|n| !self.open.contains_key(*n))
                .cloned()
                .collect(),
        )
    }

    fn score(&self, network: &Network) -> i64 {
        self.open
            .iter()
            .map(|(n, t)| network.get(n).flow_rate * (MAX_TIME - t))
            .sum()
    }

    fn optimistic_score(&self, network: &Network) -> i64 {
        network
            .nodes
            .iter()
            .map(|(k, n)| {
                let t = self.open.get(k).map(|&t| t).unwrap_or_else(|| {
                    if self.iter < ITERS {
                        network.distance(&START, k) + 1
                    } else {
                        self.time + 1
                    }
                });
                n.flow_rate * (MAX_TIME - t)
            })
            .sum()
    }
}

#[derive(Debug)]
struct DFS {
    network: Network,
    seen: BTreeMap<StateKey, (i64, i64, u8)>,
    states: BinaryHeap<(i64, i64, State)>,
}

impl DFS {
    fn new(network: Network) -> Self {
        let mut states = BinaryHeap::new();
        let init = State::new();
        states.push((init.score(&network), init.optimistic_score(&network), init));
        Self {
            network,
            seen: BTreeMap::new(),
            states,
        }
    }

    fn run(&mut self) -> Option<(i64, i64, State)> {
        let mut best = Option::None;
        while let Some((score, optimistic_score, state)) = self.states.pop() {
            if score
                > best
                    .as_ref()
                    .map(|(best_score, _, _)| *best_score)
                    .unwrap_or(0)
            {
                best = Some((score, optimistic_score, state.clone()));
                println!("New best: {} (seen {} states)", score, self.seen.len());
            }
            let state_key = state.key(&self.network);
            if let Some(&(old_time, old_score, old_iter)) = self.seen.get(&state_key) {
                if old_time <= state.time && old_score >= score && old_iter <= state.iter {
                    continue;
                }
            }

            let adj_states = state.adjacent_states(&self.network);
            self.states.extend(
                adj_states
                    .into_iter()
                    .map(|st| {
                        (
                            st.score(&self.network),
                            st.optimistic_score(&self.network),
                            st,
                        )
                    })
                    .filter(|(_score, optimistic_score, _)| {
                        *optimistic_score
                            >= best
                                .as_ref()
                                .map(|(best_score, _, _)| *best_score)
                                .unwrap_or(0)
                    }),
            );

            self.seen.insert(state_key, (state.time, score, state.iter));
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
                let valve = caps.get(1).unwrap().as_str().as_bytes();
                let flow_rate = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
                let tunnels = caps
                    .get(3)
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|t| [t.as_bytes()[0], t.as_bytes()[1]])
                    .collect::<Vec<_>>();
                network.insert([valve[0], valve[1]], RawNode { flow_rate, tunnels });
            }
        }
    }
    let network = Network::from(network);
    let mut dfs = DFS::new(network.clone());
    if let Some(best) = dfs.run() {
        println!("Best node: {:?}", best);
    } else {
        println!("Failed??");
    }
}
