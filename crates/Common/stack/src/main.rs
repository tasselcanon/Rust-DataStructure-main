use stack::LinkedStack;
use stack::Stack;
use stack::matching_brackets;
fn main() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    s.push(4);

    let mut ls = LinkedStack::new();
    ls.push(1);
    ls.push(2);
    ls.push(4);
    ls.push(8);
    ls.display();
    let a = matching_brackets("(()s)s)");
    println!("{}", a);
}
