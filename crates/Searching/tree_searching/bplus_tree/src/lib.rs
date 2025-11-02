use std::fmt::Debug;
#[derive(Debug, Clone)]
pub enum BPlusNode<T: Clone + Ord> {
    Internal {
        keys: Vec<T>,
        children: Vec<Box<BPlusNode<T>>>,
    },
    Leaf {
        keys: Vec<T>,
        next: Option<Box<BPlusNode<T>>>,
    },
}

#[derive(Debug)]
pub struct BPlusTree<T: Clone + Ord> {
    root: Option<Box<BPlusNode<T>>>,
    degree: usize,
}

impl<T: Ord + Clone + Debug> BPlusTree<T> {
    pub fn new(degree: usize) -> Self {
        Self { root: None, degree }
    }

    pub fn insert(&mut self, key: T) {
        if self.root.is_none() {
            self.root = Some(Box::new(BPlusNode::Leaf {
                keys: vec![key],
                next: None,
            }));
            return;
        }

        let root = self.root.take().unwrap();
        let (new_root, split_key, right_node) = Self::insert_internal(root, key, self.degree);

        self.root = if let Some(rnode) = right_node {
            Some(Box::new(BPlusNode::Internal {
                keys: vec![split_key.unwrap()],
                children: vec![new_root.unwrap(), rnode],
            }))
        } else {
            Some(new_root.unwrap())
        };
    }

    fn insert_internal(
        node: Box<BPlusNode<T>>,
        key: T,
        degree: usize,
    ) -> (
        Option<Box<BPlusNode<T>>>,
        Option<T>,
        Option<Box<BPlusNode<T>>>,
    ) {
        match *node {
            BPlusNode::Leaf { mut keys, mut next } => {
                match keys.binary_search(&key) {
                    Ok(_) => return (Some(Box::new(BPlusNode::Leaf { keys, next })), None, None),
                    Err(pos) => keys.insert(pos, key),
                }

                if keys.len() > degree {
                    let mid = keys.len() / 2;
                    let right_keys = keys.split_off(mid);

                    let split_key = right_keys[0].clone();

                    let right_leaf = Box::new(BPlusNode::Leaf {
                        keys: right_keys,
                        next: next.take(),
                    });

                    let left_leaf = Box::new(BPlusNode::Leaf {
                        keys,
                        next: Some(right_leaf.clone()),
                    });

                    return (Some(left_leaf), Some(split_key), Some(right_leaf));
                }

                (Some(Box::new(BPlusNode::Leaf { keys, next })), None, None)
            }

            BPlusNode::Internal {
                mut keys,
                mut children,
            } => {
                let pos = match keys.binary_search(&key) {
                    Ok(i) => i + 1,
                    Err(i) => i,
                };
                let child = children.remove(pos);
                let (new_child, split_key, right_child) = Self::insert_internal(child, key, degree);

                if let Some(nc) = new_child {
                    children.insert(pos, nc);
                }

                if let Some(sk) = split_key {
                    keys.insert(pos, sk.clone());
                    if let Some(rc) = right_child {
                        children.insert(pos + 1, rc);
                    }
                }

                if keys.len() > degree {
                    let mid = keys.len() / 2;
                    let right_keys = keys.split_off(mid + 1);
                    let split_key = keys.pop().unwrap();

                    let right_children = children.split_off(mid + 1);

                    let right_node = Box::new(BPlusNode::Internal {
                        keys: right_keys,
                        children: right_children,
                    });

                    let left_node = Box::new(BPlusNode::Internal { keys, children });

                    return (Some(left_node), Some(split_key), Some(right_node));
                }

                (
                    Some(Box::new(BPlusNode::Internal { keys, children })),
                    None,
                    None,
                )
            }
        }
    }
    pub fn display(&self) {
        if let Some(root) = &self.root {
            Self::print_node(root, 0);
        } else {
            println!("None");
        }
    }

    fn print_node(node: &Box<BPlusNode<T>>, depth: usize) {
        let indent = "    ".repeat(depth);

        match &**node {
            BPlusNode::Internal { keys, children } => {
                println!("{}[Internal - Level {}]", indent, depth);
                println!("{}{} Keys: {:?}", indent, "  ", keys);

                for child in children {
                    Self::print_node(child, depth + 1);
                }
            }

            BPlusNode::Leaf { keys, next } => {
                println!("{}[Leaf - Level {}]", indent, depth);
                println!("{}{} Keys: {:?}", indent, "  ", keys);

                let next_info = if next.is_some() {
                    " -> Connected "
                } else {
                    " -> End"
                };
                println!("{}{} {}", indent, "  ", next_info);
            }
        }
    }
}
