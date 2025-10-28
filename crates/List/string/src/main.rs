use string::SeqString;

fn main() {
    let s = SeqString::from_str("helloworld");
    let p = SeqString::from_str("owo");
    let result = s.index_kmp(&p);
    println!("{:?}", result);
}
