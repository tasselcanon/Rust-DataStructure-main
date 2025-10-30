use avl_tree::AVLTree;
fn main() {
    let mut avl = AVLTree::<i32>::new();
    for &x in &[10, 20, 30, 40, 50, 25] {
        avl.insert(x);
    }
    avl.delete(10);
    println!("{:?}", avl.search(50));
}
