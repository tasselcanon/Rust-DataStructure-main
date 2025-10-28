use std::fmt::Debug;
// 树形查找
#[derive(Debug)]
pub struct TreeNode<T> {
    pub key: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
pub struct BSTree<T> {
    pub root: Option<Box<TreeNode<T>>>,
}

impl<T> BSTree<T>
where
    T: Debug + Clone + PartialEq + PartialOrd,
{
    pub fn new_with(data: T) -> Self {
        Self {
            root: Some(Box::new(TreeNode {
                key: data,
                left: None,
                right: None,
            })),
        }
    }

    pub fn insert(&mut self, data: T) {
        fn insert_node<T: PartialOrd + Clone>(node: &mut Option<Box<TreeNode<T>>>, data: T) {
            match node {
                None => {
                    *node = Some(Box::new(TreeNode {
                        key: data,
                        left: None,
                        right: None,
                    }));
                }
                Some(n) => {
                    if data < n.key {
                        insert_node(&mut n.left, data);
                    } else {
                        insert_node(&mut n.right, data);
                    }
                }
            }
        }
        insert_node(&mut self.root, data);
    }

    pub fn delete(&mut self, data: T) {
        fn delete_node<T: PartialEq + PartialOrd + Clone>(
            node: &mut Option<Box<TreeNode<T>>>,
            data: T,
        ) -> Option<Box<TreeNode<T>>> {
            if let Some(mut n) = node.take() {
                if data < n.key {
                    n.left = delete_node(&mut n.left, data);
                    Some(n)
                } else if data > n.key {
                    n.right = delete_node(&mut n.right, data);
                    Some(n)
                } else {
                    match (n.left.take(), n.right.take()) {
                        (None, None) => None,
                        (Some(left), None) => Some(left),
                        (None, Some(right)) => Some(right),
                        (Some(_left), Some(mut right)) => {
                            let mut min = &mut right;
                            if let Some(ref mut l) = min.left {
                                min = l;
                            }
                            n.key = min.key.clone();
                            n.right = delete_node(&mut min.right, min.key.clone());
                            Some(n)
                        }
                    }
                }
            } else {
                None
            }
        }
    }
}
