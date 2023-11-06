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
    pub fn find(&mut self, x: usize) -> Option<usize> {
        if x >= self.n {
            return None;
        };
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x])?
        }
        Some(self.parent[x])
    }

    pub fn merge(&mut self, x: usize, y: usize) {
        match (self.find(x), self.find(y)) {
            (Some(x), Some(y)) => {
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
            _ => {}
        };
    }
    pub fn in_same_sets(&mut self, x: usize, y: usize) -> Option<bool> {
        match (self.find(x), self.find(y)) {
            (Some(x), Some(y)) => Some(x == y),
            (_, _) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dsu() {
        let mut dsu = DSU::new(4);
        dsu.merge(0, 1);
        assert_eq!(Some(true), dsu.in_same_sets(0, 1));
        assert_eq!(Some(false), dsu.in_same_sets(0, 2));
        dsu.merge(1, 2);
        assert_eq!(Some(true), dsu.in_same_sets(0, 2));
        assert_eq!(Some(false), dsu.in_same_sets(0, 3));
        assert_eq!(None, dsu.in_same_sets(100, 200));
    }
}
