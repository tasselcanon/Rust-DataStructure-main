use merge_sort::MergeSorter;
fn main() {
    let mut arr = [2, 5, 3, 4, 1, 6];
    MergeSorter::merge_sort(&mut arr);
    println!("{:?}", arr);
}
