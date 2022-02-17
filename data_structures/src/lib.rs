// my own basic implementation of data structures

// pub trait Summary {
    // fn size(&self) -> i32;
// }

// Queue built on top of Vector
pub mod queue {
  pub struct Queue<T> {
      items: Vec<T>,
      size: i32,
  }
  
  impl<T> Queue<T> {
      pub fn new() -> Queue<T> {
          Queue::<T> {
              items: Vec::new(),
              size: 0,
          } 
      }

      // returns size of queue
      pub fn size(&self) -> i32 {
        self.size
      }
  
      // add to the back of queue
      pub fn enqueue(&mut self, element: T) { 
          self.size += 1;
          self.items.push(element);
      }
  
      // remove from the front of queue
      pub fn dequeue(&mut self) -> Option<T> { 
          if self.size > 0 {
              self.size -= 1;
              Some(self.items.remove(0))
          }
          else { None }
      }

      // look at the front of queue
      pub fn peek(&self) -> Option<&T> { 
          if self.size > 0 {
              Some(&self.items[0])
          }
          else { None }
      }
  }
}

// Stack built on top of Vector
pub mod stack {
  pub struct Stack<T> {
      items: Vec<T>,
      size: i32,
  }

  impl<T> Stack<T> {
      pub fn new() -> Stack<T> {
          Stack::<T> {
              items: Vec::new(),
              size: 0,
          } 
      }

      pub fn size(&self) -> i32 {
        self.size
      }
  
      // add to the top of Stack
      pub fn push(&mut self, element: T) { 
          self.size += 1;
          self.items.push(element);
      }
  
      // remove from the top of Stack
      pub fn pop(&mut self) -> Option<T> { 
          if self.size > 0 {
              self.size -= 1;
              self.items.pop()
          }
          else { None }
      }

      // look at the front of queue
      pub fn peek(&self) -> Option<&T> { 
          if self.size > 0 {
              Some(&self.items[self.items.len() - 1])
          }
          else { None }
      }
  }
}

// Graph built on top of HashMap and Vectors
pub mod graph {
  pub use std::collections::HashMap;

  // graph stores vertices in hashmap indexed by unique names (strings) of the vertices
  // graph and vertices share lifetime 'a
  struct Vertex<'a, T> {
      value: T,
      // in_vertices: Vec<&'a String>,
      // out_vertices: Vec<&'a String>,
      in_vertices: Vec<&'a String>,
      out_vertices: Vec<&'a String>,
      in_degree: i32,
      out_degree: i32,
  }

  impl<'a, T> Vertex<'a, T> {
      fn new(value: T) -> Vertex<'a, T> {
          Vertex::<'a, T> {
              value,
              in_vertices: Vec::new(),
              out_vertices: Vec::new(),
              in_degree: 0,
              out_degree: 0,
          }
      }
  }

  pub struct Graph<'a, T> {
      vertices: HashMap<String, Vertex<'a, T>>,
      size: i32,
  }

  impl<'a, T> Graph<'a, T> {
      pub fn new() -> Graph<'a, T> {
          Graph::<'a, T> {
              vertices: HashMap::new(),
              size: 0,
          }
      }

      pub fn size(&self) -> i32 {
        self.size
      }

      // create a new vertex in the graph by giving it a name and value
      pub fn add_vertex(&mut self, name: String, value: T) {
          let new_vertex = Vertex::new(value);
          self.vertices.entry(name).or_insert(new_vertex);
          self.size += 1;
      }

      // add an edge from vertex named from_vertex to vertex named to_vertex
      pub fn add_edge(&mut self, from_vertex: &'a String, to_vertex: &'a String) {
          match self.vertices.get_mut(from_vertex) {
              None => (), // to introduce error handling here later
              Some(matched_vertex) => {
                  matched_vertex.out_vertices.push(&to_vertex);
                  matched_vertex.out_degree += 1;
              }
          }
          match self.vertices.get_mut(to_vertex) {
              None => (), // to introduce error handling here later
              Some(matched_vertex) => {
                  matched_vertex.in_vertices.push(&from_vertex);
                  matched_vertex.in_degree += 1;
              }
          }
      }

      pub fn delete_edge(&mut self, from_vertex: &'a String, to_vertex: &'a String) {
      //   match self.vertices.get_mut(from_vertex) {
      //     None => (), // to introduce error handling here later
      //     Some(matched_vertex) => {
      //         matched_vertex.out_vertices.push(&to_vertex);
      //         matched_vertex.out_degree += 1;
      //     }
      //   }
      //   match self.vertices.get_mut(to_vertex) {
      //     None => (), // to introduce error handling here later
      //     Some(matched_vertex) => {
      //         matched_vertex.in_vertices.push(&from_vertex);
      //         matched_vertex.in_degree += 1;
      //     }
      //   }
      // }
  }
}