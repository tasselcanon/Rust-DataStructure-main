use union_find::UnionFind;
fn main() {
    let mut uf = UnionFind::new(6);

    uf.union(1, 2);
    uf.union(2, 3);
    uf.union(4, 5);

    println!("{}", uf.connected(1, 3)); // true
    println!("{}", uf.connected(3, 5)); // false

    uf.union(3, 4);
    println!("{}", uf.connected(3, 5)); // true
}
