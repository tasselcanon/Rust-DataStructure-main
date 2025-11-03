use chaining::HashMap;

fn main() {
    let mut map = HashMap::new(7);
    map.insert(3, 10);
    map.insert(4, 12);
    map.insert(5, 14);
    map.insert(2, 80);
    map.insert(6, 30);
    map.insert(8, 50);
    map.remove(&2);
    map.display();
}
