mod data_structures;
use crate::data_structures::{queue::Queue, stack::Stack};

fn main() {
    let mut s: Stack<i32> = Stack::new();
    s.push(5);
    println!("Length of Stack: {}", s.size());
}

