use queue::SeqQueue;
use stack::Stack;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(Debug)]
struct UnionFind {
    parent: HashMap<String, String>,
}
impl UnionFind {
    fn new(vertices: impl Iterator<Item = String>) -> Self {
        let mut parent = HashMap::new();
        for v in vertices {
            parent.insert(v.clone(), v);
        }
        Self { parent }
    }

    fn find(&mut self, x: &str) -> String {
        let p = self.parent.get(x).unwrap().clone();
        if p != x {
            let root = self.find(&p);
            self.parent.insert(x.to_string(), root.clone());
            root
        } else {
            p
        }
    }

    fn union(&mut self, a: &str, b: &str) {
        let roota = self.find(a);
        let rootb = self.find(b);
        if roota != rootb {
            self.parent.insert(roota, rootb);
        }
    }
}
#[derive(Debug)]
pub struct Graph {
    pub adj_list: HashMap<String, Vec<(String, i32)>>,
    pub is_directed: bool,
}

impl Graph {
    pub fn new(is_directed: bool) -> Self {
        Self {
            adj_list: HashMap::new(),
            is_directed,
        }
    }

    pub fn add_vertex(&mut self, v: &str) {
        self.adj_list.entry(v.to_string()).or_insert(Vec::new());
    }

    pub fn add_edge(&mut self, from: &str, to: &str, weight: i32) {
        self.add_vertex(from);
        self.add_vertex(to);

        let from_neighbors = self.adj_list.entry(from.to_string()).or_default();
        if let Some((_, num)) = from_neighbors.iter_mut().find(|(s, _)| s == to) {
            *num = weight;
        } else {
            from_neighbors.push((to.to_string(), weight));
        }

        if !self.is_directed {
            let to_neighbors = self.adj_list.entry(to.to_string()).or_default();
            if let Some((_, num)) = to_neighbors.iter_mut().find(|(s, _)| s == from) {
                *num = weight;
            } else {
                to_neighbors.push((from.to_string(), weight));
            }
        }
    }

    pub fn remove_edge(&mut self, from: &str, to: &str) {
        if let Some(neighbors) = self.adj_list.get_mut(from) {
            neighbors.retain(|(k, _weight)| k != to);
        }

        if !self.is_directed {
            if let Some(neighbors) = self.adj_list.get_mut(to) {
                neighbors.retain(|(k, _weight)| k != from);
            }
        }
    }

    pub fn remove_vertex(&mut self, v: &str) {
        let maybe_neighbors = self.adj_list.remove(v);
        if let Some(neighbors) = maybe_neighbors {
            if !self.is_directed {
                for (s, _) in neighbors {
                    if let Some(nbs) = self.adj_list.get_mut(&s) {
                        nbs.retain(|(k, _)| k != v);
                    }
                }
            } else {
                for (_from, nbs) in self.adj_list.iter_mut() {
                    nbs.retain(|(k, _)| k != v);
                }
            }
        }
    }

    pub fn display(&self) {
        for (k, v) in self.adj_list.iter() {
            print!("{} -> ", k);
            for (s, n) in v {
                print!("{}({}) ", s, n);
            }
            println!();
        }
    }

    pub fn neighbors(&self, v: &str) -> Option<&Vec<(String, i32)>> {
        self.adj_list.get(v)
    }

    pub fn has_edge(&self, from: &str, to: &str) -> bool {
        if let Some(nbs) = self.adj_list.get(from) {
            nbs.iter().any(|(s, _)| s == to)
        } else {
            false
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.adj_list.len()
    }

    pub fn edge_count(&self) -> usize {
        let mut count = 0usize;

        for (_from, to) in self.adj_list.iter() {
            count += to.len();
        }
        if !self.is_directed { count / 2 } else { count }
    }

    fn dfs_recursive(&self, v: &str, visited: &mut HashSet<String>) {
        visited.insert(v.to_string());
        print!("{} ", v);
        if let Some(nbs) = self.adj_list.get(v) {
            for (s, _num) in nbs {
                if !visited.contains(s) {
                    self.dfs_recursive(s, visited);
                }
            }
        }
    }

    pub fn dfs(&self, start: &str) {
        let mut visited = HashSet::new();
        self.dfs_recursive(start, &mut visited);
    }

    pub fn dfs_stack(&self, start: &str) {
        let mut stack = Stack::new();
        let mut visited: HashSet<String> = HashSet::new();
        stack.push(start.to_string());
        while let Some(v) = stack.pop() {
            if visited.insert(v.clone()) {
                print!("{} ", v);
                if let Some(nbs) = self.adj_list.get(&v) {
                    for (s, _) in nbs.iter().rev() {
                        stack.push(s.clone());
                    }
                }
            }
        }
    }

    pub fn bfs(&self, start: &str) {
        let mut visited: HashSet<String> = HashSet::new();
        let mut q = SeqQueue::new();
        q.enqueue(start.to_string());
        while let Some(v) = q.dequeue() {
            if visited.insert(v.clone()) {
                print!("{} ", v);
                if let Some(nbs) = self.adj_list.get(&v) {
                    for (s, _) in nbs {
                        q.enqueue(s.clone());
                    }
                }
            }
        }
    }

    pub fn kruskal_mst(&self) -> Vec<(String, String, i32)> {
        let mut edges = Vec::new();
        for (u, nbs) in &self.adj_list {
            for (v, w) in nbs {
                if !self.is_directed && u > v {
                    continue;
                }
                edges.push((u.clone(), v.clone(), *w));
            }
        }
        edges.sort_by_key(|&(_, _, num)| num);
        let all_vertices = self.adj_list.keys().cloned();
        let mut uf = UnionFind::new(all_vertices);

        let mut mst = Vec::new();
        for (u, v, w) in edges {
            if uf.find(&u) != uf.find(&v) {
                uf.union(&u, &v);
                mst.push((u, v, w));
            }
        }
        mst
    }

    pub fn prim_mst(&self, start: &str) -> (Vec<(String, String, i32)>, i32) {
        let mut mst_sum = 0;
        let mut visited = HashSet::new();
        let mut mst = Vec::new();
        let mut heap = BinaryHeap::new();
        visited.insert(start.to_string());
        if let Some(nbs) = self.adj_list.get(start) {
            for (to, w) in nbs {
                heap.push(Reverse((*w, start.to_string(), to.clone())));
            }
        }
        while let Some(Reverse((weight, from, to))) = heap.pop() {
            if visited.contains(&to) {
                continue;
            }
            visited.insert(to.clone());

            mst.push((from.clone(), to.clone(), weight));
            mst_sum += weight;
            if let Some(nbs) = self.adj_list.get(&to) {
                for (next, w) in nbs {
                    heap.push(Reverse((*w, to.clone(), next.clone())));
                }
            }
        }
        (mst, mst_sum)
    }

    pub fn dijkstra(&self, start: &str) -> HashMap<String, i32> {
        let mut dist: HashMap<String, i32> = self
            .adj_list
            .keys()
            .map(|k| (k.clone(), i32::MAX))
            .collect();
        let mut heap = BinaryHeap::new();
        dist.insert(start.to_string(), 0);
        heap.push((Reverse(0), start.to_string()));

        while let Some((Reverse(d), u)) = heap.pop() {
            if d > dist[&u] {
                continue;
            }
            if let Some(nbs) = self.adj_list.get(&u) {
                for (v, w) in nbs {
                    let new_dist = *w + d;
                    if new_dist < *dist.get(v).unwrap_or(&i32::MAX) {
                        dist.insert(v.clone(), new_dist);
                        heap.push((Reverse(new_dist), v.clone()));
                    }
                }
            }
        }
        dist
    }

    pub fn topological_sort(&self) -> Option<Vec<String>> {
        // 初始化入度表
        let mut indegree: HashMap<String, usize> = HashMap::new(); // 各节点入度表
        for v in self.adj_list.keys() {
            indegree.entry(v.clone()).or_insert(0);
            for (to, _) in &self.adj_list[v] {
                *indegree.entry(to.clone()).or_insert(0) += 1;
            }
        }
        // 创建队列
        let mut q = SeqQueue::new();
        for (v, &deg) in &indegree {
            if deg == 0 {
                q.enqueue(v.clone());
            }
        }
        // 开始BFS(广度)式拓扑排序
        let mut result = Vec::new();
        while let Some(v) = q.dequeue() {
            result.push(v.clone());
            if let Some(nbs) = self.adj_list.get(&v) {
                for (to, _) in nbs {
                    if let Some(indeg) = indegree.get_mut(to) {
                        *indeg -= 1;
                        if *indeg == 0 {
                            q.enqueue(to.clone());
                        }
                    }
                }
            }
        }
        // 检查图中是否有环
        if result.len() == indegree.len() {
            Some(result)
        } else {
            None
        }
    }
}
