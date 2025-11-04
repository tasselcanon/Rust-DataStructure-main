#[derive(Debug)]
pub struct SelectionSorter;

impl SelectionSorter {
    pub fn simple_selection_sort(arr: &mut [i32]) {
        let len = arr.len();
        for i in 0..len {
            let mut min_index = i;
            for j in i..len {
                if arr[min_index] > arr[j] {
                    min_index = j;
                }
            }
            arr.swap(i, min_index);
        }
    }

    pub fn heap_sort(arr: &mut [i32]) {
        let len = arr.len();

        // 1. 构建最大堆
        for i in (0..len / 2).rev() {
            Self::heapify(arr, len, i);
        }

        // 2. 堆排序
        for i in (1..len).rev() {
            arr.swap(0, i); // 将堆顶最大值放到数组末尾
            Self::heapify(arr, i, 0); // 调整剩余堆
        }
    }

    // 调整堆，使 arr[root_index..heap_size] 满足最大堆性质
    fn heapify(arr: &mut [i32], heap_size: usize, root_index: usize) {
        let mut largest = root_index;
        let left = 2 * root_index + 1;
        let right = 2 * root_index + 2;

        if left < heap_size && arr[left] > arr[largest] {
            largest = left;
        }
        if right < heap_size && arr[right] > arr[largest] {
            largest = right;
        }

        if largest != root_index {
            arr.swap(root_index, largest);
            Self::heapify(arr, heap_size, largest);
        }
    }
}
