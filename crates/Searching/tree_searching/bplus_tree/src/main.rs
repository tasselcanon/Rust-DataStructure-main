use bplus_tree::BPlusTree;
fn main() {
    let mut tree = BPlusTree::new(3);

    let keys_to_insert = vec![10, 20, 30, 40, 50, 60, 70, 5, 15, 25];

    for key in keys_to_insert {
        tree.insert(key);
    }
    tree.display();
}
