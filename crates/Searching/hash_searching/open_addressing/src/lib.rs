use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};
#[derive(Debug, Clone)]
enum Slot<K, V> {
    Empty,
    Tombstone,
    Occupied(K, V),
}

#[derive(Debug)]
pub struct HashTab<K, V> {
    buckets: Vec<Slot<K, V>>,
    capacity: usize,
}

impl<K, V> HashTab<K, V>
where
    K: Clone + Eq + Hash + Debug,
    V: Clone + Debug,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            buckets: vec![Slot::Empty; capacity],
            capacity,
        }
    }

    pub fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.capacity as u64) as usize
    }

    pub fn insert(&mut self, key: K, value: V) {
        let mut idx = self.hash(&key);
        let start_idx = idx;
        loop {
            match &self.buckets[idx] {
                Slot::Occupied(k, _) if *k == key => {
                    self.buckets[idx] = Slot::Occupied(key, value);
                    return;
                }
                Slot::Empty => {
                    self.buckets[idx] = Slot::Occupied(key, value);
                    return;
                }
                Slot::Tombstone => {
                    self.buckets[idx] = Slot::Occupied(key, value);
                    return;
                }
                _ => {
                    idx = (idx + 1) % self.capacity;
                    if idx == start_idx {
                        println!("table full");
                        return;
                    }
                }
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut idx = self.hash(key);
        let start_idx = idx;
        loop {
            match &self.buckets[idx] {
                Slot::Occupied(k, v) if k == key => {
                    return Some(v);
                }
                Slot::Empty => return None,
                _ => {
                    idx = (idx + 1) % self.capacity;
                    if idx == start_idx {
                        return None;
                    }
                }
            }
        }
    }

    pub fn remove(&mut self, key: &K) -> bool {
        let mut idx = self.hash(key);
        let start_idx = idx;
        loop {
            match &self.buckets[idx] {
                Slot::Occupied(k, _) if k == key => {
                    self.buckets[idx] = Slot::Tombstone;
                    return true;
                }
                Slot::Empty => return false,
                _ => {
                    idx = (idx + 1) % self.capacity;
                    if idx == start_idx {
                        return false;
                    }
                }
            }
        }
    }

    pub fn display(&self) {
        for (i, value) in self.buckets.iter().enumerate() {
            match value {
                Slot::Empty => {
                    println!("{}: Empty", i);
                }
                Slot::Tombstone => {
                    println!("{}: Tombstore", i);
                }
                Slot::Occupied(k, v) => {
                    println!("{}: occupied: {:?},{:?}", i, k, v);
                }
            }
        }
    }
}
