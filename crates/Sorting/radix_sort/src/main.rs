use radix_sort::RadixSorter;
fn main() {
    let mut arr = [6, 3, 9, 5, 1, 4, 7];
    println!("before sort: {:?}", arr);
    RadixSorter::radix_sort(&mut arr);
    println!("after sort:  {:?}", arr);
}
