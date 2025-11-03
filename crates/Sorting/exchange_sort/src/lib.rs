use std::fmt::Debug;

#[derive(Debug)]
pub struct ExchangeSorter;

impl ExchangeSorter {
    pub fn bubble_sort<T: Clone + Debug + Eq + PartialOrd>(&self, arr: &mut [T]) {
        let len = arr.len();
        for i in 0..len {
            for y in i..len {
                let temp = arr[i].clone();
                if arr[y] < temp {
                    arr[i] = arr[y].clone();
                    arr[y] = temp;
                }
            }
        }
    }

    pub fn partition(arr: &mut [i32]) -> usize {
        let pivot_index = arr.len() - 1;
        let pivot = arr[pivot_index];
        let mut i = 0;
        for j in 0..pivot_index {
            if arr[j] < pivot {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, pivot_index);
        i
    }
    pub fn quick_sort(&self, arr: &mut [i32]) {
        let len = arr.len();
        if len < 1 {
            return;
        }
        let pivot_index = Self::partition(arr);
        self.quick_sort(&mut arr[..pivot_index]);
        self.quick_sort(&mut arr[pivot_index + 1..]);
    }
}
