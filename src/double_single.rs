//! # Double Singly-Linked List
//!
//! We smash our list into two halves: one going to the left, and one going to the right
//!

pub struct Stack<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}
