use exchange_sort::ExchangeSorter;
fn main() {
    let sorter = ExchangeSorter;
    let mut arr1 = [2, 4, 6, 3, 1, 5];
    let mut arr2 = [2, 4, 6, 3, 1, 5];
    sorter.bubble_sort(&mut arr1);
    sorter.quick_sort(&mut arr2);
    println!("{:?}--{:?}", arr1, arr2);
}
