use linked_list::DoublyLinkedList;
use linked_list::LNode;
fn main() {
    let mut lnode = LNode::new();
    lnode.insert_head(1);
    lnode.insert_tail(2);
    lnode.insert_tail(3);
    lnode.insert_tail(4);
    lnode.insert_index(4, 78).unwrap();
    lnode.insert_index(5, 38).unwrap();

    lnode.reverse();

    let mut current = &mut lnode;
    println!("\nlnode:");
    while let Some(ref mut next_node) = current.next {
        print!("{} ", next_node.data);
        current = current.next.as_mut().unwrap();
    }
    println!("\nlength: {}", lnode.length());

    let mut dlink: DoublyLinkedList<i32> = DoublyLinkedList {
        head: None,
        tail: None,
        length: 0,
    };
    dlink.insert_tail(1);
    dlink.insert_tail(2);
    dlink.insert_tail(3);
    dlink.insert_tail(4);
    dlink.insert_tail(5);
    dlink.insert_tail(6);
    dlink.insert_tail(7);
    println!("\ndlink:");

    let mut current = dlink.head.clone();
    for _ in 0..dlink.length {
        if let Some(node_rc) = current {
            print!("{} ", node_rc.borrow().data);
            current = node_rc.borrow().next.clone();
        }
    }
    println!("\nlength: {}", dlink.length);
}
