pub struct DSU {
    n: usize,
    rank: Vec<usize>,
    parent: Vec<usize>,
}

impl DSU {
    pub fn new(n: usize) -> DSU {
        DSU {
            n,
            rank: (0..n).collect(),
            parent: (0..n).collect(),
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        assert!(x < self.n);
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x])
        }
        self.parent[x]
    }

    pub fn merge(&mut self, x: usize, y: usize) {
        let (x, y) = (self.find(x), self.find(y));
        if x != y {
            if self.rank[x] < self.rank[y] {
                self.parent[x] = y;
            } else if self.rank[x] > self.rank[y] {
                self.parent[y] = x;
            } else {
                self.parent[x] = y;
                self.rank[x] += 1;
            }
        }
    }
    pub fn in_same_sets(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dsu() {
        let mut dsu = DSU::new(4);
        dsu.merge(0, 1);
        assert!(dsu.in_same_sets(0, 1));
        assert!(!dsu.in_same_sets(0, 2));
        dsu.merge(1, 2);
        assert!(dsu.in_same_sets(0, 2));
        assert!(!dsu.in_same_sets(0, 3))
    }
}
