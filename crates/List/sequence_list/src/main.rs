use sequence_list::SQList;

fn main() {
    let mut sqlist: SQList<i32> = SQList::new();
    sqlist.insert(1, 1).unwrap();
    sqlist.insert(2, 2).unwrap();
    sqlist.insert(3, 3).unwrap();
    sqlist.insert(4, 4).unwrap();
    sqlist.insert(1, 1).unwrap();
    sqlist.insert(2, 2).unwrap();
    sqlist.insert(3, 3).unwrap();
    sqlist.insert(4, 4).unwrap();
    println!("{:?}", sqlist);
}
