use insertion_sort::InsertionSorter;
fn main() {
    let sorter = InsertionSorter;
    let mut list = [2, 5, 7, 3, 1];
    sorter.shell_sort(&mut list);
    println!("{:?}", list);
}
