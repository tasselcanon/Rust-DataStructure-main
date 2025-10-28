use queue::SeqQueue;
use stack::Stack;
use std::{cell::RefCell, fmt::Debug, rc::Rc};
#[derive(Debug, PartialEq)]
pub struct TreeNode<T> {
    pub data: T,
    pub lchild: Option<Rc<RefCell<TreeNode<T>>>>,
    pub rchild: Option<Rc<RefCell<TreeNode<T>>>>,
}
impl<T> TreeNode<T>
where
    T: Clone + Debug + PartialEq + Default + Copy,
{
    pub fn new() -> Self {
        Self {
            data: T::default(),
            lchild: None,
            rchild: None,
        }
    }
}

#[derive(Debug)]
pub struct BiTree<T> {
    pub root: Option<Rc<RefCell<TreeNode<T>>>>,
}
impl<T> BiTree<T>
where
    T: Clone + Copy + Debug + PartialEq + Default + std::ops::Mul<usize, Output = usize>,
{
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(TreeNode {
            data,
            lchild: None,
            rchild: None,
        }));
        if self.root.is_none() {
            self.root = Some(new_node);
            return;
        }
        let mut q: SeqQueue<Rc<RefCell<TreeNode<T>>>> = SeqQueue::new();
        q.enqueue(self.root.as_ref().unwrap().clone());
        while let Some(q_node) = q.dequeue() {
            let mut q_node_borrow = q_node.borrow_mut();
            if let Some(lch_node_rc) = q_node_borrow.lchild.as_ref() {
                q.enqueue(lch_node_rc.clone());
            } else {
                q_node_borrow.lchild = Some(new_node.clone());
                return;
            }
            if let Some(rch_node_rc) = q_node_borrow.rchild.as_ref() {
                q.enqueue(rch_node_rc.clone());
            } else {
                q_node_borrow.rchild = Some(new_node.clone());
                return;
            }
        }
    }

    pub fn preorder_recursion(&self) {
        print!(" preorder: ");
        fn helper<T: std::fmt::Debug>(node: &Option<Rc<RefCell<TreeNode<T>>>>) {
            if let Some(node_rc) = node {
                let node_borrow = node_rc.borrow();
                print!("{:?} ", node_borrow.data);
                helper(&node_borrow.lchild);
                helper(&node_borrow.rchild);
            }
        }
        helper(&self.root);
        println!();
    }
    pub fn inorder_recursion(&self) {
        print!("  inorder: ");
        fn helper<T: std::fmt::Debug>(node: &Option<Rc<RefCell<TreeNode<T>>>>) {
            if let Some(node_rc) = node {
                let node_borrow = node_rc.borrow();

                helper(&node_borrow.lchild);
                print!("{:?} ", node_borrow.data);
                helper(&node_borrow.rchild);
            }
        }
        helper(&self.root);
        println!();
    }

    pub fn postorder_recursion(&self) {
        print!("postorder: ");
        fn helper<T: std::fmt::Debug>(node: &Option<Rc<RefCell<TreeNode<T>>>>) {
            if let Some(node_rc) = node {
                let node_borrow = node_rc.borrow();
                helper(&node_borrow.lchild);
                helper(&node_borrow.rchild);
                print!("{:?} ", node_borrow.data);
            }
        }
        helper(&self.root);
        println!();
    }

    pub fn preorder(&self) {
        if self.root.is_none() {
            return;
        }
        print!(" preorder: ");
        let mut s = Stack::new();
        s.push(self.root.as_ref().unwrap().clone());
        while !s.is_empty() {
            let node_rc = s.pop().unwrap();
            let node_borrow = node_rc.borrow();
            print!("{:?} ", node_borrow.data);
            if let Some(ref node) = node_borrow.rchild {
                s.push(node.clone());
            }
            if let Some(ref node) = node_borrow.lchild {
                s.push(node.clone());
            }
        }
        println!();
    }

    pub fn inorder(&self) {
        print!("  inorder: ");
        let mut s: Stack<Rc<RefCell<TreeNode<T>>>> = Stack::new();
        let mut current = self.root.clone();
        while current.is_some() || !s.is_empty() {
            while let Some(node_rc) = current.clone() {
                s.push(node_rc.clone());
                current = node_rc.borrow().lchild.clone();
            }
            if let Some(node_rc) = s.pop() {
                let node_borrow = node_rc.borrow();
                print!("{:?} ", node_borrow.data);
                current = node_borrow.rchild.clone();
            }
        }
        println!();
    }

    pub fn postorder(&self) {
        print!("postorder: ");
        let mut s = Stack::new();
        let mut current = self.root.clone();
        let mut last_visited: Option<Rc<RefCell<TreeNode<T>>>> = None;

        while !s.is_empty() || current.is_some() {
            while let Some(node_rc) = current.clone() {
                s.push(node_rc.clone());
                current = node_rc.borrow().lchild.clone();
            }
            if let Some(peek_node) = s.peek().cloned() {
                let peek_borrow = peek_node.borrow();
                let right = peek_borrow.rchild.clone();
                drop(peek_borrow);

                if right.is_none() || Some(right.clone().unwrap()) == last_visited {
                    print!("{:?} ", peek_node.borrow().data);
                    last_visited = s.pop();
                } else {
                    current = right;
                }
            }
        }
        println!();
    }

    pub fn levelorder(&self) {
        let mut q = SeqQueue::new();
        let mut current = self.root.clone();
        q.enqueue(current.clone().unwrap());
        while !q.is_empty() {
            current = q.dequeue();
            print!("{:?} ", current.as_ref().unwrap().borrow().data);
            if let Some(node_rc) = current.as_ref().unwrap().borrow().lchild.clone() {
                q.enqueue(node_rc.clone());
            }
            if let Some(node_rc) = current.as_ref().unwrap().borrow().rchild.clone() {
                q.enqueue(node_rc.clone());
            }
        }
    }

    pub fn tree_height_recursion(node: Option<Rc<RefCell<TreeNode<T>>>>) -> usize {
        match node {
            None => 0,
            Some(node_rc) => {
                let node_borrow = node_rc.borrow();
                let left_h = Self::tree_height_recursion(node_borrow.lchild.clone());
                let right_h = Self::tree_height_recursion(node_borrow.rchild.clone());
                usize::max(left_h, right_h) + 1
            }
        }
    }

    pub fn tree_height(&self) -> usize {
        let mut q = SeqQueue::new();
        q.enqueue(self.root.as_ref().unwrap().clone());
        let mut height = 0;
        while !q.is_empty() {
            let level_size = q.length();
            for _ in 0..level_size {
                let node_rc = q.dequeue().unwrap();
                let node_borrow = node_rc.borrow();
                if let Some(lnode) = node_borrow.lchild.clone() {
                    q.enqueue(lnode.clone());
                }
                if let Some(rnode) = node_borrow.rchild.clone() {
                    q.enqueue(rnode.clone());
                }
            }
            height += 1;
        }
        height
    }

    pub fn tree_width(&self) -> usize {
        if self.root.is_none() {
            return 0;
        }
        let mut max_node = 0;
        let mut q = SeqQueue::new();
        q.enqueue(self.root.as_ref().unwrap().clone());
        while !q.is_empty() {
            let level_size = q.length();
            for _ in 0..level_size {
                let node_rc = q.dequeue().unwrap();
                let node_borrow = node_rc.borrow();
                if let Some(lnode) = node_borrow.lchild.clone() {
                    q.enqueue(lnode);
                }
                if let Some(rnode) = node_borrow.rchild.clone() {
                    q.enqueue(rnode);
                }
            }
            max_node = usize::max(max_node, level_size);
        }
        max_node
    }

    pub fn delete_node(&mut self, value: T) {
        if self.root.is_none() {
            return;
        }
        let root_data_clone = self.root.as_ref().unwrap().borrow().data.clone();
        if root_data_clone == value {
            self.root = None;
            return;
        }
        let mut q = SeqQueue::new();
        q.enqueue(self.root.as_ref().unwrap().clone());
        while !q.is_empty() {
            if let Some(node_rc) = q.dequeue() {
                let node_borrow = node_rc.borrow();
                if let Some(lchild_rc) = node_borrow.lchild.clone() {
                    if lchild_rc.borrow().data.clone() == value {
                        drop(node_borrow);
                        node_rc.borrow_mut().lchild = None;
                    } else {
                        q.enqueue(lchild_rc);
                    }
                }
                let node_borrow = node_rc.borrow();
                if let Some(rchild_rc) = node_borrow.rchild.clone() {
                    if rchild_rc.borrow().data.clone() == value {
                        drop(node_borrow);
                        node_rc.borrow_mut().rchild = None;
                    } else {
                        q.enqueue(rchild_rc);
                    }
                }
            }
        }
    }

    pub fn wpl(&self) -> usize {
        if self.root.is_none() {
            return 0;
        }
        let mut q = SeqQueue::new();
        q.enqueue((self.root.as_ref().unwrap().clone(), 0usize));
        let mut wpl = 0;
        while !q.is_empty() {
            if let Some((node_rc, depth)) = q.dequeue() {
                let node_borrow = node_rc.borrow();
                let left = node_borrow.lchild.clone();
                let right = node_borrow.rchild.clone();
                if left.is_none() && right.is_none() {
                    wpl += node_borrow.data * depth;
                } else {
                    if let Some(l) = left {
                        q.enqueue((l, depth + 1));
                    }
                    if let Some(r) = right {
                        q.enqueue((r, depth + 1));
                    }
                }
            }
        }
        wpl
    }
}

#[derive(Debug, PartialEq)]
pub struct ThreadTreeNode<T> {
    pub data: T,
    pub lchild: Option<Rc<RefCell<ThreadTreeNode<T>>>>,
    pub rchild: Option<Rc<RefCell<ThreadTreeNode<T>>>>,
    pub ltag: bool,
    pub rtag: bool,
}
impl<T> ThreadTreeNode<T>
where
    T: Debug + Default + Clone,
{
    pub fn new() -> Self {
        Self {
            data: T::default(),
            lchild: None,
            rchild: None,
            ltag: true,
            rtag: true,
        }
    }

    pub fn new_with(data: T) -> Self {
        Self {
            data,
            lchild: None,
            rchild: None,
            ltag: true,
            rtag: true,
        }
    }
    pub fn insert_left(self_rc: &Rc<RefCell<Self>>, child: Rc<RefCell<ThreadTreeNode<T>>>) {
        let mut self_borrow = self_rc.borrow_mut();
        self_borrow.lchild = Some(child.clone());
        self_borrow.ltag = false;
        child.borrow_mut().lchild = Some(self_rc.clone());
    }

    pub fn insert_right(self_rc: &Rc<RefCell<Self>>, child: Rc<RefCell<ThreadTreeNode<T>>>) {
        let mut self_borrow = self_rc.borrow_mut();
        self_borrow.rchild = Some(child.clone());
        self_borrow.rtag = false;
        child.borrow_mut().rchild = None;
    }

    pub fn inorder_threading(root: Option<Rc<RefCell<Self>>>) {
        fn inthread<T: Debug>(
            node: Option<Rc<RefCell<ThreadTreeNode<T>>>>,
            pre: &mut Option<Rc<RefCell<ThreadTreeNode<T>>>>,
        ) {
            if let Some(node_rc) = node {
                let node_borrow = node_rc.borrow_mut();
                drop(node_borrow);
                inthread(node_rc.borrow().lchild.clone(), pre);

                let mut node_borrow = node_rc.borrow_mut();

                if node_borrow.lchild.is_none() {
                    node_borrow.ltag = true;
                    node_borrow.lchild = pre.clone();
                }
                if let Some(pre_rc) = pre {
                    let mut pre_borrow = pre_rc.borrow_mut();
                    if pre_borrow.rchild.is_none() {
                        pre_borrow.rtag = true;
                        pre_borrow.rchild = Some(node_rc.clone());
                    }
                }

                *pre = Some(node_rc.clone());
                drop(node_borrow);

                inthread(node_rc.borrow().rchild.clone(), pre);
            }
        }
        let mut pre: Option<Rc<RefCell<ThreadTreeNode<T>>>> = None;
        inthread(root, &mut pre);
    }
}
