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

    fn partition(arr: &mut [i32]) -> usize {
        let len = arr.len();
        let pivot_index = len - 1;
        let pivot = arr[pivot_index];
        let mut left = 0;
        let mut right = len - 1;
        while left < right {
            while left < right && arr[left] <= pivot {
                left += 1;
            }
            arr[right] = arr[left];

            while left < right && arr[right] >= pivot {
                right -= 1;
            }
            arr[left] = arr[right];
        }
        arr[left] = pivot;
        left
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
