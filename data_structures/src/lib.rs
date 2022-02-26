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

  // graph stores vertices in hashmap indexed by unique names (strings) of the vertices
  // graph and vertices share lifetime 'a
  struct Vertex<'a, T> {
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
      let new_vertex: Vertex<T> = Vertex::new(value);
      self.vertices.entry(name).or_insert(new_vertex);
      self.size += 1;
    }

    // add an edge from vertex with name from_vertex_name to vertex with name to_vertex_name
    pub fn add_edge(&mut self, from_vertex_name: &'a String, to_vertex_name: &'a String) -> Result<(), &str> {

      // check that both vertices exist
      if self.vertices.contains_key(from_vertex_name) && self.vertices.contains_key(to_vertex_name) {

        let from_vertex: &mut Vertex<'a, T> = self.vertices.get_mut(from_vertex_name).unwrap();
        if from_vertex.out_vertices.contains(&to_vertex_name) {
          return Err("from_vertex already contains to_vertex in its outgoing vertices")
        }
        else {
          from_vertex.out_vertices.push(&to_vertex_name);
        }
        
        let to_vertex: &mut Vertex<'a, T> = self.vertices.get_mut(to_vertex_name).unwrap();
        if to_vertex.in_vertices.contains(&from_vertex_name) {
          return Err("to_vertex already contains from_vertex in its incoming vertices")
        }
        else {
          to_vertex.in_vertices.push(&from_vertex_name);
        } 
      }
      else {
        return Err("one or both of the vertices do not exist in graph")
      }
      Ok(())
    }

    // delete an edge from vertex with name from_vertex_name to vertex with name to_vertex_name
    pub fn delete_edge(&mut self, from_vertex_name: &'a String, to_vertex_name: &'a String) -> Result<(), &str> {

      // check that both vertices exist
      if self.vertices.contains_key(from_vertex_name) && self.vertices.contains_key(to_vertex_name) {

        let from_vertex: &mut Vertex<'a, T> = self.vertices.get_mut(from_vertex_name).unwrap();
        if from_vertex.out_vertices.contains(&to_vertex_name) {
          let outgoing_index = from_vertex.out_vertices.iter().position(|&x| &x == &to_vertex_name).unwrap();
          from_vertex.out_vertices.remove(outgoing_index);
        }
        else {
          return Err("from_vertex does not contain to_vertex_name in its outgoing vertices")
        }

        let to_vertex: &mut Vertex<'a, T> = self.vertices.get_mut(to_vertex_name).unwrap();
        if to_vertex.out_vertices.contains(&from_vertex_name) {
          let incoming_index = to_vertex.out_vertices.iter().position(|&x| &x == &from_vertex_name).unwrap();
          to_vertex.out_vertices.remove(incoming_index);
        }
        else {
          return Err("to_vertex does not contain from_vertex_name in its incoming vertices")
        }
      }
      else {
        return Err("one or both of the vertices do not exist in graph")
      }
      Ok(())
    }
  }
}