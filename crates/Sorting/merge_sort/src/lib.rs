#[derive(Debug)]
pub struct MergeSorter;

impl MergeSorter {
    pub fn merge(arr: &mut [i32]) {
        let len = arr.len();
        let mut tmp = vec![];
        let mid = len / 2;
        let mut i = 0;
        let mut j = mid;
        while i < mid && j < len {
            if arr[i] <= arr[j] {
                tmp.push(arr[i]);
                i += 1;
            } else if arr[i] > arr[j] {
                tmp.push(arr[j]);
                j += 1;
            }
        }
        while i < mid {
            tmp.push(arr[i]);
            i += 1;
        }
        while j < len {
            tmp.push(arr[j]);
            j += 1;
        }
        arr.copy_from_slice(&tmp[..]);
    }

    pub fn merge_sort(arr: &mut [i32]) {
        let len = arr.len();
        if len <= 1 {
            return;
        }
        let mid = len / 2;
        Self::merge_sort(&mut arr[..mid]);
        Self::merge_sort(&mut arr[mid..]);
        Self::merge(arr);
    }
}
