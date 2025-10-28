use bitree::BiTree;
fn main() {
    let mut tree = BiTree::new();
    tree.insert(1);
    tree.insert(2);
    tree.insert(3);
    tree.insert(4);
    tree.insert(5);
    tree.insert(6);
    tree.insert(7);
    println!("{}", tree.wpl());
    tree.levelorder();
}

//
//             1
//       2           3
//    4     5      6     7
//
