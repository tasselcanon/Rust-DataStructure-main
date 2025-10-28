use graph::Graph;
fn main() {
    let mut g = Graph::new(true);
    g.add_edge("A", "B", 3);
    g.add_edge("A", "C", 1);
    g.add_edge("B", "C", 7);
    g.add_edge("B", "D", 5);
    g.add_edge("C", "D", 2);
    g.display();
    println!("{:?}", g.topological_sort());
}
