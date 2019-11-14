//! # Deque
//!
//! A generalized queue structure that permits insertion and removal of elements at both ends
//! where as in queue element can only be added at one end and removed from the other end.
//!
//! The queue implementation is as safe doubly linked list. This means each node has a
//! pointer to the previous and next node. Furthermore, the list itself has a pointer to the
//! first and last node.
//!

use std::cell::RefCell;
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}
