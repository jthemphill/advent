use std::io::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
enum SnailNum {
    Literal(u64),
    Nested(Box<SnailNum>, Box<SnailNum>),
}

impl SnailNum {
    fn add(left: SnailNum, right: SnailNum) -> Self {
        Self::Nested(Box::new(left), Box::new(right))
    }

    fn magnitude(&self) -> u64 {
        match self {
            Self::Literal(n) => *n,
            Self::Nested(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

#[test]
fn test_magnitude() {
    assert_eq!(SnailNum::from("[[1,2],[[3,4],5]]").magnitude(), 143);
    assert_eq!(
        SnailNum::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(),
        1384
    );
    assert_eq!(
        SnailNum::from("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(),
        445
    );
}

impl From<serde_json::Value> for SnailNum {
    fn from(val: serde_json::Value) -> SnailNum {
        match val {
            serde_json::Value::Number(num) => SnailNum::Literal(num.as_u64().unwrap()),
            serde_json::Value::Array(vals) => {
                assert_eq!(vals.len(), 2);
                let mut vals = vals.into_iter();
                SnailNum::add(
                    SnailNum::from(vals.next().unwrap()),
                    SnailNum::from(vals.next().unwrap()),
                )
            }
            _ => panic!("Not a valid snail number: {:?}", val),
        }
    }
}

impl From<&str> for SnailNum {
    fn from(line: &str) -> SnailNum {
        SnailNum::from(serde_json::from_str::<serde_json::Value>(&line).unwrap())
    }
}

fn add_to_the_left(num: SnailNum, add: u64) -> (SnailNum, u64) {
    if add == 0 {
        return (num, add);
    }
    match num {
        SnailNum::Nested(left, right) => {
            let (left, add) = add_to_the_left(*left, add);
            let (right, add) = add_to_the_left(*right, add);
            (SnailNum::add(left, right), add)
        }
        SnailNum::Literal(num) => (SnailNum::Literal(num + add), 0),
    }
}

#[test]
fn test_add_to_the_left() {
    assert_eq!(
        add_to_the_left(SnailNum::from("[[[1,2],3],4]"), 5),
        (SnailNum::from("[[[6,2],3],4]"), 0)
    );
}

#[test]
fn test_add_to_the_right() {
    assert_eq!(
        add_to_the_right(SnailNum::from("[[[1,2],3],4]"), 5),
        (SnailNum::from("[[[1,2],3],9]"), 0)
    );
}

fn add_to_the_right(num: SnailNum, add: u64) -> (SnailNum, u64) {
    if add == 0 {
        return (num, add);
    }
    match num {
        SnailNum::Nested(left, right) => {
            let (right, add) = add_to_the_right(*right, add);
            let (left, add) = add_to_the_right(*left, add);
            (SnailNum::add(left, right), add)
        }
        SnailNum::Literal(num) => (SnailNum::Literal(num + add), 0),
    }
}

fn should_immediately_explode(left: &SnailNum, right: &SnailNum, depth: i64) -> Option<(u64, u64)> {
    if depth < 4 {
        None
    } else {
        match (left, right) {
            (SnailNum::Literal(left_num), SnailNum::Literal(right_num)) => {
                Some((*left_num, *right_num))
            }
            _ => None,
        }
    }
}

/**
 * Returns the new value of `num`, and an option
 *
 * If no explosion happens in our subtree, the option will be None.
 *
 * If an explosion happens in our subtree, the option will contain
 * two numbers which need to be added higher up in the tree.
 */
fn explode_inner(num: &SnailNum, depth: i64) -> Option<(SnailNum, u64, u64)> {
    match num {
        SnailNum::Literal(_) => None,
        SnailNum::Nested(left, right) => {
            if let Some((left_num, right_num)) = should_immediately_explode(left, right, depth) {
                Some((SnailNum::Literal(0), left_num.clone(), right_num.clone()))
            } else {
                if let Some((new_left, left_add, right_add)) = explode_inner(&*left, depth + 1) {
                    let (new_right, right_add) = add_to_the_left(*right.clone(), right_add);
                    Some((SnailNum::add(new_left, new_right), left_add, right_add))
                } else if let Some((new_right, left_add, right_add)) =
                    explode_inner(&*right, depth + 1)
                {
                    let (new_left, left_add) = add_to_the_right(*left.clone(), left_add);
                    Some((SnailNum::add(new_left, new_right), left_add, right_add))
                } else {
                    None
                }
            }
        }
    }
}

fn explode(num: &SnailNum) -> Option<SnailNum> {
    match num {
        SnailNum::Literal(_) => Some(num.clone()),
        SnailNum::Nested(_, _) => {
            if let Some((new, _, _)) = explode_inner(num, 0) {
                Some(new)
            } else {
                None
            }
        }
    }
}

#[test]
fn test_explode_1() {
    assert_eq!(
        explode(&SnailNum::from("[[[[[9,8],1],2],3],4]")),
        Some(SnailNum::from("[[[[0,9],2],3],4]"))
    );
}

#[test]
fn test_explode_2() {
    assert_eq!(
        explode(&SnailNum::from("[7,[6,[5,[4,[3,2]]]]]")),
        Some(SnailNum::from("[7,[6,[5,[7,0]]]]"))
    );
}

#[test]
fn test_explode_3() {
    assert_eq!(
        explode(&SnailNum::from("[[6,[5,[4,[3,2]]]],1]")),
        Some(SnailNum::from("[[6,[5,[7,0]]],3]"))
    );
}

#[test]
fn test_explode_4() {
    assert_eq!(
        explode(&SnailNum::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")),
        Some(SnailNum::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"))
    );
}

#[test]
fn test_explode_5() {
    assert_eq!(
        explode(&SnailNum::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")),
        Some(SnailNum::from("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"))
    );
}

fn split(num: &SnailNum) -> Option<SnailNum> {
    match num {
        SnailNum::Literal(n) => {
            if *n >= 10 {
                Some(SnailNum::add(
                    SnailNum::Literal(n / 2),
                    SnailNum::Literal((n + 1) / 2),
                ))
            } else {
                None
            }
        }
        SnailNum::Nested(left, right) => {
            if let Some(splitted_left) = split(&left) {
                Some(SnailNum::add(splitted_left, *right.clone()))
            } else {
                if let Some(splitted_right) = split(&right) {
                    Some(SnailNum::add(*left.clone(), splitted_right))
                } else {
                    None
                }
            }
        }
    }
}

#[test]
fn test_split_1() {
    assert_eq!(
        split(&SnailNum::from("[[[[0,7],4],[15,[0,13]]],[1,1]]")),
        Some(SnailNum::from("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"))
    );
}

#[test]
fn test_split_2() {
    assert_eq!(
        split(&SnailNum::from("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")),
        Some(SnailNum::from("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"))
    );
}

fn reduce(mut num: SnailNum) -> SnailNum {
    loop {
        if let Some(exploded) = explode(&num) {
            num = exploded;
        } else if let Some(splitted) = split(&num) {
            num = splitted;
        } else {
            break;
        }
    }
    num
}

fn main() {
    let nums: Vec<SnailNum> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| SnailNum::from(line.unwrap().as_str()))
        .collect();
    let mut nums_iter = nums.iter().cloned();
    let mut sum = nums_iter.next().unwrap();
    for n in nums_iter {
        sum = SnailNum::add(sum, n);
        sum = reduce(sum);
    }
    sum = reduce(sum);
    println!("Magnitude of total sum: {}", sum.magnitude());

    let mut big = 0;
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }
            big = big.max(reduce(SnailNum::add(nums[i].clone(), nums[j].clone())).magnitude());
        }
    }
    println!("Greatest magnitude: {}", big);
}
