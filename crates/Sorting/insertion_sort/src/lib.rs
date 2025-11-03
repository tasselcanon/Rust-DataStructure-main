use std::fmt::Debug;

#[derive(Debug)]
pub struct InsertionSorter;

impl InsertionSorter {
    pub fn direct_insertion_sort<T: Ord + Clone + Debug>(&self, arr: &mut [T]) {
        for i in 1..arr.len() {
            let key = arr[i].clone();
            let mut j = i;
            while j > 0 && arr[j - 1] > key {
                arr[j] = arr[j - 1].clone();
                j -= 1;
            }
            arr[j] = key;
        }
    }

    pub fn binary_insertion_sort<T: Ord + Clone + Debug>(&self, arr: &mut [T]) {
        let len = arr.len();
        for i in 1..len {
            let key = arr[i].clone();
            let mut low = 0;
            let mut high = i as isize - 1;
            while low as isize <= high {
                let mid = (low + high as usize) / 2;
                if arr[mid] < key {
                    low = mid + 1;
                } else {
                    high = mid as isize - 1;
                }
            }
            let mut j = i;
            while j > low {
                arr[j] = arr[j - 1].clone();
                j -= 1;
            }
            arr[low] = key;
        }
    }

    pub fn shell_sort<T: Ord + Clone + Debug>(&self, arr: &mut [T]) {
        let n = arr.len();
        let mut gap = n / 2;
        while gap > 0 {
            for i in gap..n {
                let temp = arr[i].clone();
                let mut j = i;
                while j >= gap && arr[j - gap] > temp {
                    arr[j] = arr[j - gap].clone();
                    j -= gap;
                }
                arr[j] = temp;
            }
            gap /= 2;
        }
    }
}
