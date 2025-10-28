use std::fmt::Debug;

#[derive(Debug)]
pub struct SeqList<T> {
    data: Vec<T>,
    length: usize,
}

// 无序线性表
impl<T> SeqList<T>
where
    T: PartialEq + Clone,
{
    pub fn new(list: Vec<T>, length: usize) -> Self {
        Self { data: list, length }
    }

    pub fn seq_search(&self, key: T) -> Option<usize> {
        let mut tmp = Vec::with_capacity(self.length + 1);
        tmp.push(key.clone());
        tmp.extend_from_slice(&self.data);
        let mut i = self.length;
        while tmp[i] != key {
            i -= 1;
        }
        if i == 0 { None } else { Some(i - 1) }
    }
}

// 有序线性表
#[derive(Debug)]
pub struct OrderedSeqList<T> {
    data: Vec<T>,
    length: usize,
}

impl<T> OrderedSeqList<T>
where
    T: PartialEq + PartialOrd,
{
    pub fn new(list: Vec<T>, length: usize) -> Self {
        Self { data: list, length }
    }

    pub fn ordered_seq_search(&self, key: T) -> Option<usize> {
        for (i, val) in self.data.iter().take(self.length).enumerate() {
            if *val == key {
                return Some(i);
            } else if *val > key {
                break;
            }
        }
        None
    }

    pub fn binary_search(&self, key: T) -> Option<usize> {
        if self.length == 0 {
            return None;
        }

        let mut left = 0;
        let mut right = self.length - 1;

        while left <= right {
            let mid = (left + right) / 2;
            if self.data[mid] == key {
                return Some(mid);
            } else if self.data[mid] < key {
                left = mid + 1;
            } else {
                if mid == 0 {
                    break;
                }
                right = mid - 1;
            }
        }
        None
    }
}
