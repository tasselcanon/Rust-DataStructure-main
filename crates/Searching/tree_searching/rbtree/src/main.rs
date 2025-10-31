use rbtree::RBTree;
fn main() {
    let mut rb = RBTree::new();
    let list = vec![10, 20, 30, 15, 25, 5];
    for e in list {
        rb.insert(e);
    }
    rb.inorder();
}
