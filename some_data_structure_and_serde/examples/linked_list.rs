#![no_main]

use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;
// 保证了存储的指针指向的是一个有效的内存地址。
use std::ptr::NonNull;



struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}


impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}

pub struct LinkedList<T> { 
    length: u32, 
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    // Act like we own boxed nodes since we construct and leak them
    marker: PhantomData<Box<Node<T>>>,
}