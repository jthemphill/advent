struct File {
    nums: Vec<i64>,
    old_to_new: Vec<usize>,
    new_to_old: Vec<usize>,
}

impl File {
    fn new(nums: Vec<i64>) -> Self {
        let old_to_new: Vec<usize> = (0..nums.len()).collect();
        let new_to_old = old_to_new.clone();
        Self {
            nums,
            old_to_new,
            new_to_old,
        }
    }

    fn mix(&mut self) {
        let start = std::time::Instant::now();
        for old_i in 0..self.nums.len() {
            if old_i % 100 == 1 {
                println!(
                    "Finished {} ({:?})",
                    old_i,
                    (std::time::Instant::now() - start) / old_i as u32
                );
            }
            let val = self.nums[old_i];
            let mut new_i = self.old_to_new[old_i];
            if val < 0 {
                for _ in 0..(val.abs() as usize % self.nums.len()) {
                    new_i = self.swap_left(new_i);
                }
            } else {
                for _ in 0..(val as usize % self.nums.len()) {
                    new_i = self.swap_right(new_i);
                }
            }
        }
    }

    fn integrity_check(&self) {
        for old_i in 0..self.nums.len() {
            assert_eq!(self.new_to_old[self.old_to_new[old_i]], old_i);
        }
    }

    fn swap(&mut self, new_src: usize, new_dst: usize) -> usize {
        self.integrity_check();
        let val_src = self.nums[self.new_to_old[new_src]];
        let val_dst = self.nums[self.new_to_old[new_dst]];

        let old_src = self.new_to_old[new_src];
        let old_dst_prev = self.new_to_old[new_dst];

        // Point old_src to new_dst
        self.old_to_new[old_src] = new_dst;
        self.new_to_old[new_dst] = old_src;
        assert_eq!(new_dst, self.old_to_new[self.new_to_old[new_dst]]);
        assert_eq!(val_src, self.nums[self.new_to_old[new_dst]]);

        // Point old_dst_prev to new_src
        self.old_to_new[old_dst_prev] = new_src;
        self.new_to_old[new_src] = old_dst_prev;
        assert_eq!(new_src, self.old_to_new[self.new_to_old[new_src]]);
        assert_eq!(val_dst, self.nums[self.new_to_old[new_src]]);

        self.integrity_check();
        new_dst
    }

    fn swap_left(&mut self, new_i: usize) -> usize {
        let new_i_left = (new_i + self.nums.len() - 1) % self.nums.len();
        self.swap(new_i, new_i_left)
    }

    fn swap_right(&mut self, new_i: usize) -> usize {
        let new_i_right = (new_i + 1) % self.nums.len();
        self.swap(new_i, new_i_right)
    }

    fn out(&self) -> Vec<i64> {
        self.new_to_old
            .iter()
            .map(|&old_i| self.nums[old_i])
            .collect()
    }
}

struct NewFile {
    nums: Vec<i64>,
    prev: Vec<usize>,
    next: Vec<usize>,
}

impl NewFile {
    fn new(nums: Vec<i64>) -> Self {
        let prev = (0..nums.len())
            .map(|n| (n + nums.len() - 1) % nums.len())
            .collect();
        let next = (0..nums.len()).map(|n| (n + 1) % nums.len()).collect();
        Self { nums, prev, next }
    }

    fn mix(&mut self) {
        self.integrity_check();
        for (src, val) in self.nums.iter().cloned().enumerate() {
            // Find destination node. We will add after this node.
            let mut dst = src;
            if val <= 0 {
                // Go backwards one extra time, because we'll add after the destination node
                for _ in 0..(val.abs() as usize % self.nums.len() + 1) {
                    dst = self.prev[dst];
                }
            } else {
                for _ in 0..(val.abs() as usize % self.nums.len()) {
                    dst = self.next[dst];
                }
            }
            if src != dst {
                // Remove source node
                self.next[self.prev[src]] = self.next[src];
                self.prev[self.next[src]] = self.prev[src];
                // Add source node after destination node
                self.next[src] = self.next[dst];
                self.prev[self.next[dst]] = src;
                self.next[dst] = src;
                self.prev[src] = dst;
            }
        }
        self.integrity_check();
    }

    fn integrity_check(&self) {
        for i in 0..self.nums.len() {
            assert_eq!(self.prev[self.next[i]], i);
            assert_eq!(self.next[self.prev[i]], i);
        }
    }

    fn out(&self) -> Vec<i64> {
        let mut p = 0;
        let mut ret = vec![];
        for _ in 0..self.nums.len() {
            ret.push(self.nums[p]);
            p = self.next[p];
        }
        assert_eq!(p, 0);
        assert_eq!(ret.len(), self.nums.len());
        ret
    }
}

fn main() {
    let mut nums: Vec<i64> = vec![];
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            nums.push(line.parse().unwrap());
        }
    }
    let mut file = NewFile::new(nums);
    file.mix();
    let out = file.out();
    println!("{:?}", out);
    let (zero, _) = out.iter().enumerate().find(|(_, n)| **n == 0).unwrap();

    let coords: Vec<i64> = [1000, 2000, 3000]
        .into_iter()
        .map(|shift| out[(zero + shift) % out.len()])
        .collect();
    println!("Coords: {:?}", coords);
    println!("Coords: {}", coords.into_iter().sum::<i64>());
}

#[test]
fn test_swap_one_zero() {
    let mut f = File::new(vec![1, 0]);
    assert_eq!(f.swap(1, 0), 0);
    assert_eq!(f.old_to_new, vec![1, 0]);
    assert_eq!(f.new_to_old, vec![1, 0]);
    assert_eq!(f.out(), vec![0, 1]);
}

#[test]
fn test_swap_zero_one() {
    let mut f = File::new(vec![1, 0]);
    f.swap(0, 1);
    assert_eq!(f.old_to_new, vec![1, 0]);
    assert_eq!(f.new_to_old, vec![1, 0]);
    assert_eq!(f.out(), vec![0, 1]);
}

#[test]
fn test_swap_left() {
    let mut f = File::new(vec![1, 0]);
    f.swap_left(0);
    assert_eq!(f.old_to_new, vec![1, 0]);
    assert_eq!(f.new_to_old, vec![1, 0]);
    assert_eq!(f.out(), vec![0, 1]);
}

#[test]
fn test_swap_right() {
    let mut f = File::new(vec![1, 0]);
    f.swap_right(1);
    assert_eq!(f.old_to_new, vec![1, 0]);
    assert_eq!(f.new_to_old, vec![1, 0]);
    assert_eq!(f.out(), vec![0, 1]);
}

#[test]
fn test_negative_mixing() {
    let mut f = File::new(vec![0, -1]);
    f.mix();
    assert_eq!(f.out(), vec![-1, 0]);
}

#[test]
fn test_negative_mod_mixing() {
    let mut f = File::new(vec![0, -3]);
    f.mix();
    assert_eq!(f.out(), vec![-3, 0]);
}
