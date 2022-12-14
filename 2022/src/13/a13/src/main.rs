use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
enum Tok {
    Open,
    Num(i32),
    Close,
}

#[derive(Debug, Eq, PartialEq)]
enum V {
    Num(i32),
    List(Vec<V>),
}

impl Ord for V {
    fn cmp(&self, other: &V) -> Ordering {
        match self {
            V::Num(n1) => match other {
                V::Num(n2) => n1.cmp(n2),
                V::List(_) => V::List(vec![V::Num(*n1)]).cmp(other),
            },
            V::List(v1) => match other {
                V::Num(n2) => self.cmp(&V::List(vec![V::Num(*n2)])),
                V::List(v2) => {
                    for i in 0..v1.len().min(v2.len()) {
                        match v1[i].cmp(&v2[i]) {
                            Ordering::Greater => {
                                return Ordering::Greater;
                            }
                            Ordering::Less => {
                                return Ordering::Less;
                            }
                            Ordering::Equal => {}
                        }
                    }
                    v1.len().cmp(&v2.len())
                }
            },
        }
    }
}

impl PartialOrd for V {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn tokenize(rui: &[u8]) -> Vec<Tok> {
    let mut tokens = vec![];
    let mut current_number = Option::None;
    for c in rui {
        match c {
            b'[' => {
                tokens.push(Tok::Open);
            }
            b']' => {
                if let Some(num) = current_number.take() {
                    tokens.push(Tok::Num(num));
                }
                tokens.push(Tok::Close);
            }
            b',' => {
                if let Some(num) = current_number.take() {
                    tokens.push(Tok::Num(num));
                }
            }
            &c => {
                if b'0' <= c && c <= b'9' {
                    let n = (c - b'0') as i32;
                    if let Some(num) = &mut current_number {
                        *num *= 10;
                        *num += n;
                    } else {
                        current_number = Some(n);
                    }
                } else {
                    panic!("Unknown character: {}", c as char);
                }
            }
        }
    }
    tokens
}

fn parse_list(mut rui: &[Tok]) -> (V, &[Tok]) {
    let mut v_list = vec![];
    while !rui.is_empty() {
        match rui[0] {
            Tok::Open => {
                let (v, new_rui) = parse_list(&rui[1..]);
                v_list.push(v);
                rui = new_rui;
            }
            Tok::Close => {
                return (V::List(v_list), &rui[1..]);
            }
            Tok::Num(n) => {
                v_list.push(V::Num(n));
                rui = &rui[1..];
            }
        };
    }
    panic!("No closing brace found");
}

fn parse(rui: &[Tok]) -> V {
    match rui[0] {
        Tok::Open => {
            let (v, rui) = parse_list(&rui[1..]);
            if !rui.is_empty() {
                panic!("V: {:?}, Remaining Unparsed Input: {:?}", v, rui);
            }
            v
        }
        Tok::Close => panic!("Unexpected ]"),
        Tok::Num(n) => V::Num(n),
    }
}

fn main() {
    let mut pairs = vec![];
    let mut unpaired = Option::None;
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            if line.len() > 0 {
                let tokens = tokenize(&line.as_bytes());
                let v = parse(&tokens);
                if let Some(v1) = unpaired.take() {
                    pairs.push((v1, v));
                } else {
                    unpaired = Some(v);
                }
            }
        }
    }
    let mut index_total = 0;
    for (i, (v1, v2)) in pairs.iter().enumerate() {
        match v1.cmp(&v2) {
            Ordering::Equal => {}
            Ordering::Greater => {}
            Ordering::Less => {
                index_total += i + 1;
            }
        }
    }
    println!("Total: {}", index_total);
}
