#![no_main]

use std::collections::LinkedList;



#[derive(Debug)]
pub struct Queue<T> { 
    elements: LinkedList<T>,
}

impl <T> Queue <T> {
    pub fn new () -> Queue<T> { 
        Queue { elements:  LinkedList::new(), }
    }

    pub fn enqueue(&mut self, value: T){
        self.elements.push_back(value)
    }

    pub fn  dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.elements.front()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue::new()
    }
}

