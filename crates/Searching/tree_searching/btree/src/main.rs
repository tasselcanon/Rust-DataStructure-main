use btree::BTree;
fn main() {
    let mut btree = BTree::new(2);

    let data = vec![10, 20, 5, 6, 12, 30, 7, 17];
    for (_, &key) in data.iter().enumerate() {
        btree.insert(key);
    }
    btree.bt_print();
}
