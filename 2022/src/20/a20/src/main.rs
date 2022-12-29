struct File {
    nums: Vec<i64>,
    prev: Vec<usize>,
    next: Vec<usize>,
}

impl File {
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
            // Remove source node
            self.next[self.prev[src]] = self.next[src];
            self.prev[self.next[src]] = self.prev[src];

            // Find destination node. We will add after this node.
            let mut dst = src;
            if val <= 0 {
                // Go backwards one extra time, because we'll add after the destination node
                for _ in 0..(val.abs() + 1) {
                    dst = self.prev[dst];
                }
            } else {
                for _ in 0..(val.abs()) {
                    dst = self.next[dst];
                }
            }
            // Add source node after destination node
            self.next[src] = self.next[dst];
            self.prev[self.next[dst]] = src;
            self.next[dst] = src;
            self.prev[src] = dst;
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
    let mut file = File::new(nums);
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
