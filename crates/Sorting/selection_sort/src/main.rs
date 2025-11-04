use selection_sort::SelectionSorter;
fn main() {
    let mut data = vec![9, 3, 7, 1, 5, 8, 2];
    println!("Before sorting: {:?}", data);
    SelectionSorter::heap_sort(&mut data);
    println!("After sorting:  {:?}", data);
}
