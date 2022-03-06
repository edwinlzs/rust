fn main() {
    test_graph();
}

fn test_graph() {
    use data_structures::graph::Graph;

    let mut g: Graph = Graph::new();
    g.add_vertex(String::from("A"));
    g.add_vertex(String::from("B"));
    g.add_vertex(String::from("C"));
    println!("Size of Graph: {}", g.size());
    let from = String::from("A");
    let to = String::from("B");
    g.add_edge(&from, &to, 5);
    let from = String::from("B");
    let to = String::from("C");
    g.add_edge(&from, &to, 8);
    g.print_graph();

    for (key, value) in g.get_incoming_edges(String::from("A")) {
      println!("{}:{}", key, value);
    }
}

// fn test_stack() {
//     use data_structures::stack::Stack;
//
//     let mut s: Stack<i32> = Stack::<i32>::new();
//     s.push(5);
//     println!("Length of Stack: {}", s.size());
// }
