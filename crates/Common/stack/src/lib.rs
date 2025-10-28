use std::fmt::Debug;

// 顺序栈
#[derive(Debug)]
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }
}

// 链栈
#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct LinkedStack<T> {
    pub top: Option<Box<Node<T>>>,
    pub size: usize,
}

impl<T> LinkedStack<T>
where
    T: PartialEq + Clone + Copy + Debug,
{
    pub fn new() -> Self {
        Self { top: None, size: 0 }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Node {
            data: value,
            next: self.top.take(),
        };
        self.top = Some(Box::new(new_node));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            let Node { data, next } = *node;
            self.top = next;
            self.size -= 1;
            data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| &node.data)
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn length(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        while self.top.is_some() {
            self.pop();
        }
    }

    pub fn display(&self) {
        let mut current = self.top.as_ref();
        while let Some(node) = current {
            print!("{:?} ", node.data);
            current = node.next.as_ref();
        }
        println!("\nlength: {}", self.size);
    }
}

pub fn matching_brackets(s: &str) -> bool {
    let mut stack: Stack<char> = Stack::new();
    for c in s.chars() {
        match c {
            '(' => {
                stack.push(c);
            }
            ')' => {
                if stack.is_empty() {
                    return false;
                }
                stack.pop();
            }
            _ => continue,
        }
    }
    stack.is_empty()
}
