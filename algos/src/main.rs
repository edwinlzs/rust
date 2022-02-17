fn main() {
    test_graph();
}

fn test_graph() {
    use data_structures::graph::Graph;

    let mut g: Graph<i32> = Graph::<i32>::new();
    g.add_vertex(String::from("A"));
    g.add_vertex(String::from("B"));
    g.add_vertex(String::from("C"));
    println!("Size of Graph: {}", g.size());
}

// fn test_stack() {
//     use data_structures::stack::Stack;
//
//     let mut s: Stack<i32> = Stack::<i32>::new();
//     s.push(5);
//     println!("Length of Stack: {}", s.size());
// }
