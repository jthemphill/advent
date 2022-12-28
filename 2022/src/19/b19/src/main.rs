use rayon::prelude::*;
use regex::Regex;
use std::collections::{BinaryHeap, HashSet};

const MAX_TIME: usize = 32;

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Goods {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Goods {
    fn contains(&self, other: &Goods) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
    }

    fn add(&self, other: &Goods) -> Self {
        let ore = self.ore + other.ore;
        let clay = self.clay + other.clay;
        let obsidian = self.obsidian + other.obsidian;
        let geode = self.geode + other.geode;
        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }

    fn subtract(&self, other: &Goods) -> Self {
        let ore = self.ore - other.ore;
        let clay = self.clay - other.clay;
        let obsidian = self.obsidian - other.obsidian;
        let geode = self.geode - other.geode;
        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Blueprint {
    id: usize,
    ore: Goods,
    clay: Goods,
    obsidian: Goods,
    geode: Goods,
}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct State {
    time: usize,
    resources: Goods,
    bots: Goods,
}

impl State {
    fn init() -> Self {
        State {
            time: 0,
            resources: Default::default(),
            bots: Goods {
                ore: 1,
                ..Default::default()
            },
        }
    }

    fn next(&self, bp: &Blueprint) -> Vec<Self> {
        let mut next_states = vec![Self {
            time: self.time + 1,
            resources: self.resources.add(&self.bots),
            bots: self.bots,
        }];
        macro_rules! buy_bot {
            ($resource_name:ident) => {
                if self.resources.contains(&bp.$resource_name) {
                    next_states.push(Self {
                        time: self.time + 1,
                        resources: self.resources.subtract(&bp.$resource_name).add(&self.bots),
                        bots: Goods {
                            $resource_name: self.bots.$resource_name + 1,
                            ..self.bots
                        },
                    });
                }
            };
        }
        buy_bot!(ore);
        buy_bot!(clay);
        buy_bot!(obsidian);
        buy_bot!(geode);
        return next_states;
    }
}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Approx {
    optimistic_num_geodes: usize,
    state: State,
}

impl Approx {
    fn new(_bp: &Blueprint, state: State) -> Self {
        let mut optimistic_num_geodes = state.resources.geode;
        let mut optimistic_num_geode_bots = state.bots.geode;
        for _ in state.time..MAX_TIME {
            optimistic_num_geodes += optimistic_num_geode_bots;
            optimistic_num_geode_bots += 1;
        }
        Self {
            optimistic_num_geodes,
            state,
        }
    }
}

fn evaluate(bp: &Blueprint) -> usize {
    let mut frontier = BinaryHeap::new();
    frontier.push(Approx::new(bp, State::init()));
    let mut seen = HashSet::new();
    let mut best_num_geodes = 0;
    while let Some(approx) = frontier.pop() {
        if approx.optimistic_num_geodes <= best_num_geodes {
            continue;
        }
        let state = approx.state;
        if !seen.insert(state) {
            continue;
        }
        best_num_geodes = best_num_geodes.max(state.resources.geode);
        if state.time < MAX_TIME {
            for state in state.next(bp) {
                frontier.push(Approx::new(bp, state));
            }
        }
    }
    best_num_geodes
}

fn main() {
    let re = Regex::new(
        r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.",
    ).unwrap();
    let mut blueprints = vec![];
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let caps = re.captures(&line).unwrap();
            blueprints.push(Blueprint {
                id: caps.get(1).unwrap().as_str().parse().unwrap(),
                ore: Goods {
                    ore: caps.get(2).unwrap().as_str().parse().unwrap(),
                    ..Default::default()
                },
                clay: Goods {
                    ore: caps.get(3).unwrap().as_str().parse().unwrap(),
                    ..Default::default()
                },
                obsidian: Goods {
                    ore: caps.get(4).unwrap().as_str().parse().unwrap(),
                    clay: caps.get(5).unwrap().as_str().parse().unwrap(),
                    ..Default::default()
                },
                geode: Goods {
                    ore: caps.get(6).unwrap().as_str().parse().unwrap(),
                    obsidian: caps.get(7).unwrap().as_str().parse().unwrap(),
                    ..Default::default()
                },
            });
        }
    }
    let multiplied_geodes: usize = blueprints
        .par_iter()
        .take(3)
        .map(|bp| {
            let num_geodes = evaluate(bp);
            let quality = bp.id * num_geodes;
            println!("Blueprint {}: {} (quality {})", bp.id, num_geodes, quality);
            num_geodes
        })
        .product();
    println!("Multiplied together: {}", multiplied_geodes);
}
