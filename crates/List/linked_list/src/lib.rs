use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

/// 单向链表
#[derive(Debug)]
pub struct LNode<T> {
    pub data: T,
    pub next: Option<Box<LNode<T>>>,
}

impl<T> LNode<T>
where
    T: Clone + Copy + PartialEq + Default,
{
    pub fn new() -> Self {
        Self {
            data: T::default(),
            next: None,
        }
    }

    pub fn insert_tail(&mut self, data: T) {
        let mut current = self;
        while let Some(ref mut next_node) = current.next {
            current = next_node;
        }
        current.next = Some(Box::new(LNode { data, next: None }));
    }

    pub fn insert_head(&mut self, data: T) {
        let new_node = Self {
            data,
            next: self.next.take(),
        };
        self.next = Some(Box::new(new_node))
    }

    pub fn insert_index(&mut self, index: usize, data: T) -> Result<(), &'static str> {
        let mut count = 0;
        let mut current = self;
        while let Some(ref mut _next_node) = current.next {
            if count == index {
                let new_node = Self {
                    data,
                    next: current.next.take(),
                };
                current.next = Some(Box::new(new_node));
                return Ok(());
            } else {
                current = current.next.as_mut().unwrap();
                count += 1;
            }
        }
        if count == index {
            let new_node = Self {
                data,
                next: current.next.take(),
            };
            current.next = Some(Box::new(new_node));
            return Ok(());
        }
        Err("index invaild")
    }

    pub fn length(&self) -> usize {
        let mut length = 0;
        let mut current = &self.next;
        while let Some(node) = current {
            current = &node.next;
            length += 1;
        }
        length
    }

    pub fn locate(&self, data: T) -> Option<&LNode<T>> {
        let mut current = self;
        while let Some(next_node) = &current.next {
            if next_node.data == data {
                return Some(next_node);
            }
            current = &next_node;
        }
        None
    }

    pub fn delete_value(&mut self, value: T) -> Result<(), &'static str> {
        let mut current = self;
        while let Some(ref mut next_node) = current.next {
            if next_node.data == value {
                current.next = next_node.next.take();
                return Ok(());
            } else {
                current = current.next.as_mut().unwrap();
            }
        }
        Err("Not value")
    }

    pub fn delete_index(&mut self, index: usize) -> Result<(), &'static str> {
        let mut current = self;
        let mut count = 0;
        while let Some(ref mut next_node) = current.next {
            if count == index {
                current.next = next_node.next.take();
                return Ok(());
            } else {
                current = current.next.as_mut().unwrap();
                count += 1;
            }
        }
        Err("index invaild")
    }

    pub fn reverse(&mut self) {
        let mut pre: Option<Box<LNode<T>>> = None;
        let mut cur = self.next.take();
        while let Some(mut node) = cur {
            let next_node = node.next.take();
            node.next = pre.take();
            pre = Some(node);
            cur = next_node;
        }
        self.next = pre;
    }
}

/// 双向链表
#[derive(Debug, Clone)]
pub struct DNode<T> {
    pub data: T,
    pub next: Option<Rc<RefCell<DNode<T>>>>,
    pub prev: Option<Weak<RefCell<DNode<T>>>>,
}
impl<T> DNode<T>
where
    T: Clone + Copy + PartialEq + Default,
{
    pub fn new() -> Self {
        Self {
            data: T::default(),
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    pub head: Option<Rc<RefCell<DNode<T>>>>,
    pub tail: Option<Rc<RefCell<DNode<T>>>>,
    pub length: usize,
}

impl<T> DoublyLinkedList<T>
where
    T: Clone + Copy + PartialEq + Default + PartialOrd + Debug,
{
    pub fn insert_head(&mut self, data: T) {
        if let Some(old_head_rc) = self.head.take() {
            let new_head_rc = Rc::new(RefCell::new(DNode {
                next: Some(old_head_rc.clone()),
                prev: None,
                data,
            }));
            old_head_rc.borrow_mut().prev = Some(Rc::downgrade(&new_head_rc));
            self.head = Some(new_head_rc.clone());
        } else {
            let new_head_rc = Rc::new(RefCell::new(DNode {
                next: None,
                prev: None,
                data,
            }));
            self.head = Some(new_head_rc.clone());
            self.tail = Some(new_head_rc.clone());
        }
        self.length += 1;
    }

    pub fn insert_tail(&mut self, data: T) {
        if let Some(old_tail_rc) = self.tail.take() {
            let new_tail_rc = Rc::new(RefCell::new(DNode {
                data,
                next: None,
                prev: Some(Rc::downgrade(&old_tail_rc)),
            }));
            self.tail = Some(new_tail_rc.clone());
            old_tail_rc.borrow_mut().next = Some(new_tail_rc.clone());
        } else {
            let new_tail_rc = Rc::new(RefCell::new(DNode {
                data,
                next: None,
                prev: None,
            }));
            self.head = Some(new_tail_rc.clone());
            self.tail = Some(new_tail_rc.clone());
        }
        self.length += 1;
    }

    pub fn insert_index(&mut self, data: T, index: usize) -> Result<(), &'static str> {
        let mut count = 0;
        let mut current = self.head.clone();
        if index == 0 {
            self.insert_head(data);
            return Ok(());
        }
        if index == self.length {
            self.insert_tail(data);
            return Ok(());
        }
        while let Some(node_rc) = current {
            if index == count {
                let mut node_borrow = node_rc.borrow_mut();
                let prev_weak = node_borrow.prev.as_ref().unwrap();
                let prev_rc = prev_weak.upgrade().unwrap();
                let new_node_rc = Rc::new(RefCell::new(DNode {
                    data,
                    next: Some(node_rc.clone()),
                    prev: Some(Rc::downgrade(&prev_rc)),
                }));
                node_borrow.prev = Some(Rc::downgrade(&new_node_rc));
                prev_rc.borrow_mut().next = Some(new_node_rc.clone());
                self.length += 1;
                return Ok(());
            } else {
                current = node_rc.borrow().next.clone();
                count += 1;
            }
        }

        Err("index invaild")
    }

    pub fn delete_index(&mut self, index: usize) -> Result<(), &'static str> {
        if self.length == 0 {
            return Err("index invaild");
        }
        if index == 0 {
            if self.length == 1 {
                self.head = None;
                self.tail = None;
                self.length = 0;
            } else {
                let node_rc = self.head.take().unwrap();
                let next_rc = node_rc.borrow_mut().next.take().unwrap();
                next_rc.borrow_mut().prev = None;
                self.head = Some(next_rc);
                self.length -= 1;
            }
            return Ok(());
        }
        if index == self.length - 1 {
            let node_rc = self.tail.take().unwrap();
            let prev_rc = node_rc.borrow_mut().prev.take().unwrap().upgrade().unwrap();
            prev_rc.borrow_mut().next = None;
            self.tail = Some(prev_rc);
            self.length -= 1;
            return Ok(());
        }
        let mut current = self.head.clone();
        let mut count = 0;
        while let Some(node_rc) = current {
            if index == count {
                let mut node_borrow = node_rc.borrow_mut();
                let next_rc = node_borrow.next.clone().unwrap();
                let prev_rc = node_borrow.prev.as_ref().unwrap().upgrade().unwrap();
                next_rc.borrow_mut().prev = Some(Rc::downgrade(&prev_rc));
                prev_rc.borrow_mut().next = Some(next_rc.clone());
                node_borrow.next = None;
                node_borrow.prev = None;
                self.length -= 1;
                return Ok(());
            } else {
                current = node_rc.borrow().next.clone();
                count += 1;
            }
        }
        Err("index invaild")
    }

    pub fn delete_value_once(&mut self, value: T) -> Result<(), &'static str> {
        let mut current = self.head.clone();
        for _ in 0..self.length {
            if let Some(node_rc) = current.clone() {
                let node_data = node_rc.borrow().data.clone();
                if node_data == value {
                    if Rc::ptr_eq(&node_rc, self.head.as_ref().unwrap()) {
                        let old_head_rc = self.head.take().unwrap();
                        let new_head_rc = old_head_rc.borrow_mut().next.take().unwrap();
                        new_head_rc.borrow_mut().prev = None;
                        self.head = Some(new_head_rc);
                    } else if Rc::ptr_eq(&node_rc, self.tail.as_ref().unwrap()) {
                        let old_tail_rc = self.tail.take().unwrap();
                        let new_tail_rc = old_tail_rc
                            .borrow_mut()
                            .prev
                            .take()
                            .unwrap()
                            .upgrade()
                            .unwrap();
                        new_tail_rc.borrow_mut().next = None;
                        self.tail = Some(new_tail_rc);
                    } else {
                        let mut node_borrow = node_rc.borrow_mut();
                        let prev_rc = node_borrow.prev.as_ref().unwrap().upgrade().unwrap();
                        let next_rc = node_borrow.next.clone().unwrap();
                        next_rc.borrow_mut().prev = Some(Rc::downgrade(&prev_rc));
                        prev_rc.borrow_mut().next = Some(next_rc.clone());
                        node_borrow.next = None;
                        node_borrow.prev = None;
                    }
                    self.length -= 1;
                    return Ok(());
                } else {
                    current = node_rc.borrow().next.clone();
                }
            }
        }
        Err("invaild value")
    }

    pub fn delete_value_all(&mut self, value: T) {
        let mut current = self.head.clone();
        while let Some(ref node_rc) = current {
            let node_data = node_rc.borrow().data;
            let next_node = node_rc.borrow().next.clone();
            if node_data == value {
                if Rc::ptr_eq(&node_rc, self.head.as_ref().unwrap()) {
                    let new_head_rc = node_rc.borrow_mut().next.take().unwrap();
                    new_head_rc.borrow_mut().prev = None;
                    self.head = Some(new_head_rc);
                } else if Rc::ptr_eq(&node_rc, self.tail.as_ref().unwrap()) {
                    let prev_weak = node_rc.borrow().prev.clone();
                    {
                        let mut n = node_rc.borrow_mut();
                        n.next = None;
                        n.prev = None;
                    }
                    let prev_rc = prev_weak.unwrap().upgrade().unwrap();
                    prev_rc.borrow_mut().next = None;
                    self.tail = Some(prev_rc);
                } else {
                    let next_rc = node_rc.borrow_mut().next.take().unwrap();
                    let prev_rc = node_rc.borrow_mut().prev.take().unwrap().upgrade().unwrap();
                    prev_rc.borrow_mut().next = Some(next_rc.clone());
                    next_rc.borrow_mut().prev = Some(Rc::downgrade(&prev_rc));
                }
                self.length -= 1;
            }
            current = next_node;
        }
    }

    pub fn make_circular(&mut self) -> Result<(), &'static str> {
        if self.length == 0 {
            return Err("list empty");
        } else {
            let tail = self.tail.take().unwrap();
            let head = self.head.take().unwrap();
            tail.borrow_mut().next = Some(head.clone());
            head.borrow_mut().prev = Some(Rc::downgrade(&tail));
            self.head = Some(head);
            self.tail = Some(tail);
        }
        Ok(())
    }

    pub fn get_common(list1: &mut DoublyLinkedList<T>, list2: &mut DoublyLinkedList<T>) -> Self {
        let mut current1 = list1.head.clone();
        let mut current2 = list2.head.clone();

        let mut list3 = DoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
        };

        while current1.is_some() && current2.is_some() {
            match (current1.clone(), current2.clone()) {
                (Some(cur1_rc), Some(cur2_rc)) => {
                    let cur1_data = cur1_rc.borrow().data;
                    let cur2_data = cur2_rc.borrow().data;
                    if cur1_data > cur2_data {
                        current2 = cur2_rc.borrow().next.clone();
                    } else if cur1_data < cur2_data {
                        current1 = cur1_rc.borrow().next.clone();
                    } else {
                        list3.insert_tail(cur1_data);
                        current1 = cur1_rc.borrow().next.clone();
                        current2 = cur2_rc.borrow().next.clone();
                    }
                }
                _ => break,
            }
        }

        list3
    }

    pub fn symmetry(&self) -> bool {
        if self.length == 0 {
            return false;
        } else if self.length == 1 {
            return true;
        } else {
            let mut head = self.head.clone();
            let mut tail = self.tail.clone();
            for _ in 0..self.length / 2 {
                if let (Some(head_rc), Some(tail_rc)) = (head.clone(), tail.clone()) {
                    let head_data = head_rc.borrow().data;
                    let tail_data = tail_rc.borrow().data;
                    if head_data == tail_data {
                        head = head_rc.borrow().next.clone();
                        tail = tail_rc.borrow().prev.as_ref().and_then(|w| w.upgrade());
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn has_cycle(&self) -> bool {
        let mut fast = self.head.as_ref().and_then(|rc| rc.borrow().next.clone());
        let mut slow = self.head.clone();

        while let (Some(fast_rc), Some(slow_rc)) = (fast.clone(), slow.clone()) {
            if Rc::ptr_eq(&fast_rc, &slow_rc) {
                return true;
            } else {
                let next1 = fast_rc.borrow().next.clone();
                if let Some(next1_rc) = next1 {
                    let next2 = next1_rc.borrow().next.clone();
                    fast = next2;
                }
                slow = slow_rc.borrow().next.clone();
            }
        }
        false
    }
}
