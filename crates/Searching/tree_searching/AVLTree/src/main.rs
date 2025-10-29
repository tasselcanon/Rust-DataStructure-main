use AVLTree::AVLTree;
fn main() {
    let mut avl = AVLTree::<i32>::new();
    for &x in &[10, 20, 30, 40, 50, 25] {
        avl.insert(x);
    }
    println!("{:?}", avl.inorder());
}
