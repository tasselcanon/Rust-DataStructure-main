use huffman_tree::build_huffman;

fn main() {
    let num = vec![4, 6, 2, 3];
    println!("{:?}", build_huffman(num));
}
