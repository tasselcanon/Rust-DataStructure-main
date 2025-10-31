use std::fmt::Debug;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Color {
    Red,
    Black,
}

#[derive(Debug)]
pub struct RBNode<T> {
    key: T,
    color: Color,
    left: Option<Box<RBNode<T>>>,
    right: Option<Box<RBNode<T>>>,
}

#[derive(Debug)]
pub struct RBTree<T> {
    root: Option<Box<RBNode<T>>>,
}

impl<T> RBTree<T>
where
    T: Debug + Clone + PartialOrd + PartialEq,
{
    pub fn new() -> Self {
        Self { root: None }
    }

    fn is_red(node: &Option<Box<RBNode<T>>>) -> bool {
        node.as_ref().is_some_and(|n| n.color == Color::Red)
    }

    fn rotate_left(mut h: Box<RBNode<T>>) -> Box<RBNode<T>> {
        let mut y = h.right.take().expect("no left child");
        h.right = y.left.take();
        y.color = h.color.clone();
        h.color = Color::Red;
        y.left = Some(h);
        y
    }

    fn rotate_right(mut h: Box<RBNode<T>>) -> Box<RBNode<T>> {
        let mut y = h.left.take().expect("no left child");
        h.left = y.right.take();
        y.color = h.color.clone();
        h.color = Color::Red;
        y.right = Some(h);
        y
    }

    fn flip_color(node: &mut Box<RBNode<T>>) {
        node.color = match node.color {
            Color::Black => Color::Red,
            Color::Red => Color::Black,
        };
        if let Some(left) = node.left.as_mut() {
            left.color = match left.color {
                Color::Black => Color::Red,
                Color::Red => Color::Black,
            };
        }
        if let Some(right) = node.right.as_mut() {
            right.color = match right.color {
                Color::Black => Color::Red,
                Color::Red => Color::Black,
            };
        }
    }

    fn insert_node(node: Option<Box<RBNode<T>>>, key: T) -> Option<Box<RBNode<T>>> {
        match node {
            None => Some(Box::new(RBNode {
                key,
                color: Color::Red,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                if key < n.key {
                    n.left = Self::insert_node(n.left.take(), key);
                } else if key > n.key {
                    n.right = Self::insert_node(n.right.take(), key);
                }

                // 右子为红 => 左旋
                if Self::is_red(&n.right) {
                    n = Self::rotate_left(n);
                }
                // 左子为红 && 左左为红 => 右旋
                if Self::is_red(&n.left) && Self::is_red(&n.left.as_ref().unwrap().left) {
                    n = Self::rotate_right(n);
                }
                // 左右均红
                if Self::is_red(&n.left) && Self::is_red(&n.right) {
                    Self::flip_color(&mut n);
                }

                Some(n)
            }
        }
    }

    pub fn insert(&mut self, key: T) {
        self.root = Self::insert_node(self.root.take(), key);
        if let Some(ref mut n) = self.root {
            n.color = Color::Black;
        }
    }

    fn inorder_node(node: &Option<Box<RBNode<T>>>) {
        if let Some(n) = node {
            Self::inorder_node(&n.left);
            print!("{:?}:{:?} ", n.key, n.color);
            Self::inorder_node(&n.right);
        }
    }
    pub fn inorder(&self) {
        Self::inorder_node(&self.root);
    }
}
