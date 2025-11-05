#[derive(Debug)]
pub struct RadixSorter;
impl RadixSorter {
    fn max_digits(arr: &[u32]) -> u32 {
        let max_val = arr.iter().copied().max().unwrap_or(0);
        if max_val == 0 {
            return 1;
        }
        ((max_val as f64).log10().floor() as u32) + 1
    }

    pub fn radix_sort(arr: &mut [u32]) {
        let max_digits = Self::max_digits(arr);
        let mut exp = 1;
        for _ in 0..max_digits {
            let mut buckets: Vec<Vec<u32>> = vec![Vec::new(); 10];
            for &num in arr.iter() {
                let digit = (num / exp % 10) as usize;
                buckets[digit].push(num);
            }
            let mut idx = 0;
            for bucket in buckets {
                for num in bucket {
                    arr[idx] = num;
                    idx += 1;
                }
            }
            exp *= 10;
        }
    }
}
