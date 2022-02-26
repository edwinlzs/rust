// my own basic implementation of data structures

// pub trait Summary {
    // fn size(&self) -> i32;
// }

// dev mode allow unused code
#![allow(dead_code)]
#![allow(unused_variables)]

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

  // graph stores vertices in hashmap indexed by unique names of the vertices
  // graph and vertices share lifetime 'a
  struct Vertex<'a, T> {
      // stores edges info as incoming vertices and outgoing vertices
      in_vertices: HashMap<&'a String, T>,
      out_vertices: HashMap<&'a String, T>,
      in_degree: i32,
      out_degree: i32,
  }

  impl<'a, T: Copy> Vertex<'a, T> {
    fn new() -> Vertex<'a, T> {
      Vertex::<'a, T> {
          in_vertices: HashMap::new(),
          out_vertices: HashMap::new(),
          in_degree: 0,
          out_degree: 0,
      }
    }
  }

  pub struct Graph<'a, T> {
    vertices: HashMap<String, Vertex<'a, T>>,
    size: i32,
  }

  impl<'a, T: Copy> Graph<'a, T> {
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
    pub fn add_vertex(&mut self, name: String) {
      let new_vertex: Vertex<T> = Vertex::new();
      self.vertices.entry(name).or_insert(new_vertex);
      self.size += 1;
    }

    // add an edge from vertex with name from_vertex_name to vertex with name to_vertex_name
    pub fn add_edge(&mut self, from: &'a String, to: &'a String, cost: T) -> Result<(), &str> {

      // check that both vertices exist
      if self.vertices.contains_key(from) && self.vertices.contains_key(to) {

        let from_vertex: &mut Vertex<'a, T> = self.vertices.get_mut(from).unwrap();
        if from_vertex.out_vertices.contains_key(to) {
          return Err("from_vertex already contains to_vertex in its outgoing vertices")
        }
        else {
          from_vertex.out_vertices.entry(to).or_insert(cost);
        }
        
        let to_vertex: &mut Vertex<'a, T> = self.vertices.get_mut(to).unwrap();
        if to_vertex.in_vertices.contains_key(from) {
          return Err("to_vertex already contains from_vertex in its incoming vertices")
        }
        else {
          to_vertex.in_vertices.entry(from).or_insert(cost);
        } 
      }
      else {
        return Err("one or both of the vertices do not exist in graph")
      }
      Ok(())
    }

    // delete an edge from vertex with name from_vertex_name to vertex with name to_vertex_name
    pub fn delete_edge(&mut self, from: &'a String, to: &'a String) -> Result<(), &str> {

      // check that both vertices exist
      if self.vertices.contains_key(from) && self.vertices.contains_key(to) {

        let from_vertex: &mut Vertex<'a, T> = self.vertices.get_mut(from).unwrap();
        if from_vertex.out_vertices.contains_key(&to) {
          from_vertex.out_vertices.remove(&to);
        }
        else {
          return Err("from_vertex does not contain to_vertex in its outgoing vertices")
        }

        let to_vertex: &mut Vertex<'a, T> = self.vertices.get_mut(to).unwrap();
        if to_vertex.out_vertices.contains_key(&from) {
          to_vertex.out_vertices.remove(&from);
        }
        else {
          return Err("to_vertex does not contain from_vertex in its incoming vertices")
        }
      }
      else {
        return Err("one or both of the vertices do not exist in graph")
      }
      Ok(())
    }
  }
}