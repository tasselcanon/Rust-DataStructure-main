use std::{cell::RefCell, collections::BinaryHeap, rc::Rc};

#[derive(Debug, Clone)]
pub struct Node {
    weight: usize,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(weight: usize) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Self {
            weight,
            left: None,
            right: None,
        }))
    }
}

pub struct Wrapper(pub Rc<RefCell<Node>>);

impl PartialEq for Wrapper {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Wrapper {}

impl Ord for Wrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.borrow().weight.cmp(&self.0.borrow().weight)
    }
}

impl PartialOrd for Wrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn build_huffman(weights: Vec<usize>) -> Rc<RefCell<Node>> {
    let mut heap = BinaryHeap::new();
    for w in weights {
        heap.push(Wrapper(Node::new(w)));
    }

    while heap.len() > 1 {
        let n1 = heap.pop().unwrap().0;
        let n2 = heap.pop().unwrap().0;
        let parent = Node::new(n1.borrow().weight + n2.borrow().weight);
        parent.borrow_mut().left = Some(n1.clone());
        parent.borrow_mut().right = Some(n2.clone());
        heap.push(Wrapper(parent.clone()));
    }
    heap.pop().unwrap().0
}
