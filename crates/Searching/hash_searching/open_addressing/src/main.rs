use open_addressing::HashTab;
fn main() {
    let mut map = HashTab::new(7);
    map.insert(2, 13);
    map.insert(4, 14);
    map.insert(9, 18);
    map.display();
    println!("{:?}", map.remove(&2));
    map.display();
}
