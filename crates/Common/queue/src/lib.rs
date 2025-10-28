use std::{collections::VecDeque, fmt::Debug};

// 顺序队列
#[derive(Debug)]
pub struct SeqQueue<T> {
    data: VecDeque<T>,
}

impl<T> SeqQueue<T>
where
    T: Debug + Clone,
{
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.data.push_back(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.front()
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn display(&self) {
        for data in &self.data {
            print!("{:?} ", data);
        }
        println!("\nlength: {}", self.length());
    }
}

// 链式队列
#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T>
where
    T: Default,
{
    pub fn new() -> Self {
        Self {
            data: T::default(),
            next: None,
        }
    }
}

pub struct LinkedQueue<T> {
    front: Option<Box<Node<T>>>,
    rear: Option<*mut Node<T>>,
    size: usize,
}
impl<T> LinkedQueue<T>
where
    T: Clone + Copy + PartialEq + Debug,
{
    pub fn new() -> Self {
        Self {
            front: None,
            rear: None,
            size: 0,
        }
    }

    pub fn enqueue(&mut self, value: T) {
        let new_node = Box::new(Node {
            data: value,
            next: None,
        });
        if self.front.is_none() {
            self.front = Some(new_node);
            self.rear = Some(self.front.as_deref_mut().unwrap() as *mut Node<T>);
        } else {
            unsafe {
                if let Some(node) = self.rear {
                    let raw = Box::into_raw(new_node);
                    (*node).next = Some(Box::from_raw(raw));
                    self.rear = Some(raw);
                }
            }
        }
        self.size += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.front.take().map(|mut node| {
            self.front = node.next.take();
            self.size -= 1;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.front.as_deref().map(|node| &node.data)
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn clear(&mut self) {
        while self.front.is_some() {
            self.dequeue();
        }
    }

    pub fn display(&self) {
        print!("data: ");
        let mut current = self.front.as_deref();
        while let Some(node) = current {
            print!("{:?} ", node.data);
            current = node.next.as_deref();
        }
        println!("\nlength: {}", self.len());
    }
}

impl<T: Debug> Debug for LinkedQueue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vec = Vec::new();
        let mut current = self.front.as_deref();
        while let Some(node) = current {
            vec.push(&node.data);
            current = node.next.as_deref();
        }
        f.debug_struct("LinkedQueue")
            .field("data", &vec)
            .field("size", &self.size)
            .finish()
    }
}
