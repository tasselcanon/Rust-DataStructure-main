use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct AVLNode<T: PartialOrd + Clone> {
    pub key: T,
    pub left: Option<Box<AVLNode<T>>>,
    pub right: Option<Box<AVLNode<T>>>,
    pub height: i32,
}
#[derive(Debug)]
pub struct AVLTree<T: PartialOrd + Clone> {
    pub root: Option<Box<AVLNode<T>>>,
}

#[allow(dead_code)]
impl<T: PartialOrd + Clone + Debug> AVLTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, key: T) {
        self.root = Self::insert_node(self.root.take(), key);
    }

    fn height(node: &Option<Box<AVLNode<T>>>) -> i32 {
        node.as_ref().map_or(0, |n| n.height)
    }

    fn update_height(node: &mut Box<AVLNode<T>>) {
        let hl = Self::height(&node.left);
        let hr = Self::height(&node.right);
        node.height = 1 + i32::max(hl, hr);
    }

    fn balance_factor(node: &Option<Box<AVLNode<T>>>) -> i32 {
        if let Some(n) = node {
            Self::height(&n.left) - Self::height(&n.right)
        } else {
            0
        }
    }

    fn rotate_right(mut y: Box<AVLNode<T>>) -> Box<AVLNode<T>> {
        let mut x = y.left.take().expect("no left child");
        y.left = x.right.take();
        Self::update_height(&mut y);
        Self::update_height(&mut x);
        x.right = Some(y);
        x
    }

    fn rotate_left(mut x: Box<AVLNode<T>>) -> Box<AVLNode<T>> {
        let mut y = x.right.take().expect("not right child");
        x.right = y.left.take();
        Self::update_height(&mut x);
        Self::update_height(&mut y);
        y.left = Some(x);
        y
    }

    fn insert_node(node: Option<Box<AVLNode<T>>>, key: T) -> Option<Box<AVLNode<T>>> {
        match node {
            None => Some(Box::new(AVLNode {
                key,
                left: None,
                right: None,
                height: 1,
            })),
            Some(mut n) => {
                if key < n.key {
                    n.left = Self::insert_node(n.left.take(), key);
                } else if key > n.key {
                    n.right = Self::insert_node(n.right.take(), key);
                } else {
                    return Some(n);
                }
                Self::update_height(&mut n);
                let balance = Self::height(&n.left) - Self::height(&n.right);
                if balance > 1 && {
                    let left_ref = n.left.as_ref().unwrap();
                    Self::height(&left_ref.left) >= Self::height(&left_ref.right)
                } {
                    return Some(Self::rotate_right(n));
                }
                if balance > 1 {
                    let left_ref = n.left.as_mut().unwrap();
                    if Self::height(&left_ref.left) < Self::height(&left_ref.right) {
                        n.left = n.left.take().map(|left_node| Self::rotate_left(left_node));
                        return Some(Self::rotate_right(n));
                    }
                }
                if balance < -1 && {
                    let right_ref = n.right.as_ref().unwrap();
                    Self::height(&right_ref.left) <= Self::height(&right_ref.right)
                } {
                    return Some(Self::rotate_left(n));
                }
                if balance < -1 {
                    let right_ref = n.right.as_mut().unwrap();
                    if Self::height(&right_ref.left) > Self::height(&right_ref.right) {
                        n.right = n
                            .right
                            .take()
                            .map(|right_node| Self::rotate_right(right_node));
                        return Some(Self::rotate_left(n));
                    }
                }
                Some(n)
            }
        }
    }

    fn inorder_node(node: &Option<Box<AVLNode<T>>>) {
        if let Some(n) = node {
            Self::inorder_node(&n.left);
            print!("{:?} ", n.key);

            Self::inorder_node(&n.right);
        }
    }
    pub fn inorder(&self) {
        Self::inorder_node(&self.root);
    }

    fn delete_node(node: Option<Box<AVLNode<T>>>, key: T) -> Option<Box<AVLNode<T>>> {
        let mut n = node?;
        if key < n.key {
            n.left = Self::delete_node(n.left.take(), key);
        } else if key > n.key {
            n.right = Self::delete_node(n.right.take(), key);
        } else {
            match (n.left.take(), n.right.take()) {
                (None, None) => return None,
                (Some(left_node), None) => return Some(left_node),
                (None, Some(right_node)) => return Some(right_node),
                (Some(left_node), Some(mut right_node)) => {
                    let mut min = &mut right_node;
                    while let Some(ref mut l) = min.left {
                        min = l;
                    }
                    n.key = min.key.clone();
                    n.right = Self::delete_node(Some(right_node), n.key.clone());

                    n.left = Some(left_node);
                }
            }
        }
        Self::update_height(&mut n);
        let balance = Self::height(&n.left) - Self::height(&n.right);
        if balance > 1 {
            if Self::height(&n.left.as_ref().unwrap().left)
                > Self::height(&n.right.as_ref().unwrap().right)
            {
                return Some(Self::rotate_right(n));
            } else {
                n.left = n.left.take().map(|r| Self::rotate_left(r));
                return Some(Self::rotate_right(n));
            }
        }

        if balance < -1 {
            if Self::height(&n.left.as_ref().unwrap().left)
                < Self::height(&n.right.as_ref().unwrap().right)
            {
                return Some(Self::rotate_left(n));
            } else {
                n.right = n.right.take().map(|r| Self::rotate_right(r));
                return Some(Self::rotate_left(n));
            }
        }

        Some(n)
    }
    pub fn delete(&mut self, key: T) {
        self.root = Self::delete_node(self.root.take(), key);
    }

    fn search_node<'a>(node: &'a Option<Box<AVLNode<T>>>, key: &T) -> Option<&'a T> {
        if let Some(n) = node {
            if &n.key > key {
                Self::search_node(&n.left, key)
            } else if &n.key < key {
                Self::search_node(&n.right, key)
            } else {
                Some(&n.key)
            }
        } else {
            None
        }
    }

    pub fn search(&self, key: T) -> Option<&T> {
        Self::search_node(&self.root, &key)
    }
}
