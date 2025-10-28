#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<i32>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: vec![-1; n],
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] < 0 {
            return x;
        }
        let root = self.find(self.parent[x] as usize);
        self.parent[x] = root as i32;
        root
    }
    pub fn union(&mut self, x: usize, y: usize) {
        let rootx = self.find(x);
        let rooty = self.find(y);
        if rootx != rooty {
            if self.parent[rootx] < self.parent[rooty] {
                self.parent[rooty] = rootx as i32;
            } else if self.parent[rootx] > self.parent[rooty] {
                self.parent[rootx] = rooty as i32;
            } else {
                self.parent[rootx] = rooty as i32;
                self.parent[rooty] -= 1;
            }
        }
    }
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
