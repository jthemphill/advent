use regex::Regex;

#[derive(Debug)]
enum Op {
    Plus(usize),
    Times(usize),
    Square,
}

#[derive(Debug, Default)]
struct MonkeyBuilder {
    id: Option<usize>,
    items: Option<Vec<usize>>,
    operator: Option<Op>,
    test_div: Option<usize>,
    throw_true: Option<usize>,
    throw_false: Option<usize>,
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<usize>,
    operator: Op,
    test_div: usize,
    throw_true: usize,
    throw_false: usize,
}

impl From<MonkeyBuilder> for Monkey {
    fn from(builder: MonkeyBuilder) -> Monkey {
        Monkey {
            id: builder.id.unwrap(),
            items: builder.items.unwrap(),
            operator: builder.operator.unwrap(),
            test_div: builder.test_div.unwrap(),
            throw_true: builder.throw_true.unwrap(),
            throw_false: builder.throw_false.unwrap(),
        }
    }
}

impl Monkey {
    fn inspect(&mut self) -> (Vec<usize>, Vec<usize>, usize) {
        let mut true_items = vec![];
        let mut false_items = vec![];
        let num_inspected = self.items.len();
        for &item in &self.items {
            let item = self.worry(item);
            if item % self.test_div == 0 {
                true_items.push(item);
            } else {
                false_items.push(item);
            }
        }
        self.items.clear();
        (true_items, false_items, num_inspected)
    }

    fn worry(&self, old: usize) -> usize {
        println!("  Monkey inspects an item with a worry level of {}.", old);
        let new = match self.operator {
            Op::Plus(rhs) => old + rhs,
            Op::Times(rhs) => old * rhs,
            Op::Square => old * old,
        };
        let new = new / 3;
        println!(
            "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
            new,
        );
        new
    }
}

fn main() {
    let monkey_re = Regex::new(r"Monkey (\d+):").unwrap();
    let items_re = Regex::new(r"  Starting items: ([0-9, ]+)").unwrap();
    let op_re = Regex::new(r"  Operation: new = old (\*|\+) (\d+|old)").unwrap();
    let test_re = Regex::new(r"  Test: divisible by (\d+)").unwrap();
    let true_re = Regex::new(r"    If true: throw to monkey (\d+)").unwrap();
    let false_re = Regex::new(r"   If false: throw to monkey (\d+)").unwrap();

    let mut monkeys = vec![];
    let mut cur = MonkeyBuilder::default();
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                monkeys.push(Monkey::from(std::mem::take(&mut cur)));
            } else {
                if let Some(caps) = monkey_re.captures(&line) {
                    cur.id = Some(caps.get(1).unwrap().as_str().parse::<usize>().unwrap());
                } else if let Some(caps) = items_re.captures(&line) {
                    cur.items = Some(
                        caps.get(1)
                            .unwrap()
                            .as_str()
                            .split(", ")
                            .map(|i| i.parse::<usize>().unwrap())
                            .collect::<Vec<_>>(),
                    )
                } else if let Some(caps) = op_re.captures(&line) {
                    let rhs = caps.get(2).unwrap().as_str();
                    let op = match caps.get(1).unwrap().as_str() {
                        "*" => match rhs {
                            "old" => Op::Square,
                            _ => Op::Times(rhs.parse::<usize>().unwrap()),
                        },
                        "+" => Op::Plus(rhs.parse::<usize>().unwrap()),
                        x => panic!("Expected an operator, got {}", x),
                    };
                    cur.operator = Some(op);
                } else if let Some(caps) = test_re.captures(&line) {
                    cur.test_div = Some(caps.get(1).unwrap().as_str().parse::<usize>().unwrap());
                } else if let Some(caps) = true_re.captures(&line) {
                    cur.throw_true = Some(caps.get(1).unwrap().as_str().parse().unwrap());
                } else if let Some(caps) = false_re.captures(&line) {
                    cur.throw_false = Some(caps.get(1).unwrap().as_str().parse().unwrap());
                } else {
                    panic!("Couldn't parse line: {}", line)
                }
            }
        }
    }
    monkeys.push(Monkey::from(cur));

    let mut inspected_totals = vec![0; monkeys.len()];
    for round in 1..=20 {
        for i in 0..monkeys.len() {
            println!("Monkey {}:", i);
            let (true_items, false_items, num_inspected) = monkeys[i].inspect();
            let throw_true = monkeys[i].throw_true;
            let throw_false = monkeys[i].throw_false;
            monkeys[throw_true].items.extend(true_items.into_iter());
            monkeys[throw_false].items.extend(false_items.into_iter());
            inspected_totals[i] += num_inspected;
        }
        println!(
            "After round {}, the monkeys are holding items with these worry levels:",
            round
        );
        for monkey in &monkeys {
            println!("Monkey {}: {:?}", monkey.id, monkey.items);
        }
    }

    inspected_totals.sort();
    println!(
        "MB: {}",
        inspected_totals[inspected_totals.len() - 1] * inspected_totals[inspected_totals.len() - 2]
    );
}
