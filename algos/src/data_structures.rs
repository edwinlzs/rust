// my own basic implementation of data structures built on top of Vectors

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
    
        pub fn enqueue(&mut self, element: T) { // add to the back of queue
            self.size += 1;
            self.items.push(element);
        }
    
        pub fn dequeue(&mut self) -> Option<T> { // remove from the front of queue
            if self.size > 0 {
                self.size -= 1;
                Some(self.items.remove(0))
            }
            else { None }
        }

        pub fn peek(&self) -> Option<&T> { // look at the front of queue
            if self.size > 0 {
                Some(&self.items[0])
            }
            else { None }
        }
    
        pub fn size(&self) -> &i32 { // retrieve length of queue
            &self.size
        }
    }
}

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
    
        pub fn push(&mut self, element: T) { // add to the top of Stack
            self.size += 1;
            self.items.push(element);
        }
    
        pub fn pop(&mut self) -> Option<T> { // remove from the top of Stack
            if self.size > 0 {
                self.size -= 1;
                self.items.pop()
            }
            else { None }
        }

        pub fn peek(&self) -> Option<&T> { // look at the front of queue
            if self.size > 0 {
                Some(&self.items[self.items.len() - 1])
            }
            else { None }
        }
    
        pub fn size(&self) -> &i32 { // retrieve length of queue
            &self.size
        }
    }
}

pub mod graph {
    struct Vertex<T> {
        name: String,
        value: T,
        in_vertices: Vec<Vertex<T>>,
        out_vertices: Vec<Vertex<T>>,
        in_degree: i32,
        out_degree: i32,
    }

    impl<T> Vertex<T> {
        fn new(name: String, value: T) -> Vertex<T> {
            Vertex::<T> {
                name,
                value,
                in_vertices: Vec::new(),
                out_vertices: Vec::new(),
                in_degree: 0,
                out_degree: 0,
            }
        }
    }

    pub struct Graph<T> {
        vertices: Vec<Vertex<T>>,
        size: i32,
    }

    impl<T> Graph<T> {
        pub fn new() -> Graph<T> {
            Graph::<T> {
                vertices: Vec::new(),
                size: 0,
            }
        }

        pub fn add_vertex(&mut self, name: String, value: T) {
            let new_vertex = Vertex::new(name, value);
            self.vertices.push(new_vertex);
        }
    }
}