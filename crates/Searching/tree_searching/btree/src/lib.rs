use std::fmt::Debug;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct BTreeNode<T> {
    pub keys: Vec<T>,
    pub children: Vec<Box<BTreeNode<T>>>,
    pub is_leaf: bool,
}

#[derive(Debug)]
pub struct BTree<T> {
    pub root: Option<Box<BTreeNode<T>>>,
    pub t: usize, // 最小度数
}

impl<T> BTree<T>
where
    T: Debug + Clone + PartialEq + PartialOrd,
{
    pub fn new(t: usize) -> Self {
        Self { root: None, t }
    }

    fn search_node<'a>(n: &'a BTreeNode<T>, key: &T) -> Option<&'a T> {
        let mut i = 0;
        while i < n.keys.len() && key > &n.keys[i] {
            i += 1;
        }

        if i < n.keys.len() && key == &n.keys[i] {
            return Some(&n.keys[i]);
        } else if n.is_leaf {
            return None;
        } else {
            return Self::search_node(&n.children[i], key);
        }
    }

    pub fn search(&self, key: &T) -> Option<&T> {
        Self::search_node(&self.root.as_ref().unwrap(), key)
    }

    pub fn split_child(parent: &mut BTreeNode<T>, index: usize, t: usize) {
        let full_child = parent.children[index].as_mut();
        let mid_key = full_child.keys[t - 1].clone();
        let new_node = BTreeNode {
            keys: full_child.keys[t..].to_vec(),
            children: {
                if full_child.is_leaf {
                    vec![]
                } else {
                    full_child.children[t..].to_vec()
                }
            },
            is_leaf: full_child.is_leaf,
        };
        full_child.keys = full_child.keys[..t - 1].to_vec();
        if !full_child.is_leaf {
            full_child.children = full_child.children[..t].to_vec();
        }
        parent.keys.insert(index, mid_key);
        parent.children.insert(index + 1, Box::new(new_node));
    }

    fn insert_nonfull(node: &mut BTreeNode<T>, key: T, t: usize) {
        let mut i = node.keys.len() - 1;
        if node.is_leaf {
            while i > 0 && key < node.keys[i] {
                i -= 1;
            }
            node.keys.insert(i, key);
        } else {
            while i > 0 && key < node.keys[i] {
                i -= 1;
            }
            i += 1;
            if node.children[i].keys.len() == 2 * t - 1 {
                Self::split_child(node, i, t);
                if key > node.keys[i] {
                    i += 1;
                }
            }
            Self::insert_nonfull(&mut *node.children[i], key, t);
        }
    }

    pub fn insert(&mut self, key: T) {
        if self.root.is_none() {
            // 空树，创建根
            self.root = Some(Box::new(BTreeNode {
                keys: vec![key],
                children: vec![],
                is_leaf: true,
            }));
            return;
        }

        let t = self.t;
        let root_full;
        {
            let root = self.root.as_ref().unwrap();
            root_full = root.keys.len() == 2 * t - 1;
        }

        if root_full {
            let old_root = self.root.take().unwrap(); // 拿出旧根
            let mut new_root = Box::new(BTreeNode {
                keys: vec![],
                children: vec![old_root],
                is_leaf: false,
            });
            Self::split_child(&mut new_root, 0, t);
            Self::insert_nonfull(&mut new_root, key, t);
            self.root = Some(new_root);
        } else {
            let root = self.root.as_mut().unwrap();
            Self::insert_nonfull(root, key, t);
        }
    }

    pub fn bt_print_node(node: &BTreeNode<T>, level: usize) {
        println!(
            "{:indent$}level {}: {:?}",
            "",
            level,
            node.keys,
            indent = level * 4
        );

        if !node.is_leaf {
            for child in &node.children {
                Self::bt_print_node(child, level + 1);
            }
        }
    }

    pub fn bt_print(&self) {
        if let Some(root) = &self.root {
            Self::bt_print_node(root, 0);
        } else {
            println!("空树");
        }
    }
}
