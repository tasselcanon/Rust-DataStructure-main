use queue::LinkedQueue;
// use queue::SeqQueue;
fn main() {
    // let mut queue = SeqQueue::new();
    // queue.enqueue(23);
    // queue.enqueue(12);
    // queue.enqueue(45);
    // queue.display();

    let mut linkqueue = LinkedQueue::new();
    linkqueue.enqueue(2);
    linkqueue.enqueue(3);
    linkqueue.enqueue(4);
    linkqueue.display();
}
