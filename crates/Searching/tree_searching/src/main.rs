use tree_searching::BSTree;

fn main() {
    let mut bst = BSTree::new_with(3);
    bst.insert(10);
    bst.insert(5);
    bst.insert(15);
    bst.insert(12);
    bst.insert(18);
    println!("{:?}", bst);

    bst.delete(15);
    println!("After delete 15: {:?}", bst);
}
