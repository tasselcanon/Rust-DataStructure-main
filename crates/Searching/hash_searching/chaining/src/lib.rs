use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

#[derive(Debug, Clone)]
pub struct HashNode<K, V> {
    key: K,
    value: V,
}

#[derive(Debug, Clone)]
pub struct HashMap<K, V> {
    buckets: Vec<Vec<HashNode<K, V>>>,
    capacity: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Clone + Debug + PartialEq + Hash,
    V: Clone + Debug,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            buckets: vec![Vec::new(); capacity],
            capacity,
        }
    }

    pub fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.capacity as u64) as usize
    }

    pub fn insert(&mut self, key: K, value: V) {
        let idx = self.hash(&key);

        let bucket = &mut self.buckets[idx];
        for node in bucket.iter_mut() {
            if node.key == key {
                node.value = value;
                return;
            }
        }
        bucket.push(HashNode { key, value });
    }

    pub fn remove(&mut self, key: &K) {
        let idx = self.hash(key);
        let bucket = &mut self.buckets[idx];
        if let Some(pos) = bucket.iter().position(|node| node.key == *key) {
            bucket.remove(pos);
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let idx = self.hash(key);
        self.buckets[idx]
            .iter()
            .find(|node| node.key == *key)
            .map(|node| &node.value)
    }

    pub fn display(&self) {
        for (index, value) in self.buckets.iter().enumerate() {
            print!("{}: ", index);
            for i in value {
                print!("[{:?}: {:?}] ", i.key, i.value);
            }
            println!();
        }
    }
}
