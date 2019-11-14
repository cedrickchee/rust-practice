//! # Deque
//!
//! A generalized queue structure that permits insertion and removal of elements at both ends
//! where as in queue element can only be added at one end and removed from the other end.
//!
//! The queue implementation is as safe doubly linked list. This means each node has a
//! pointer to the previous and next node. Furthermore, the list itself has a pointer to the
//! first and last node.
//!

use std::cell::{Ref, RefCell, RefMut};
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

impl<T> Node<T> {
    pub fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head: None,
            tail: None,
        }
    }

    /// Pushing onto the front of the list.
    /// It specially handle some boundary cases around empty lists.
    ///
    /// Most operations will only touch the head or tail pointer. However when transitioning to or
    /// from the empty list, it edit both at once.
    ///
    /// Each node should have exactly two pointers to it. Each node in the middle of the list is
    /// pointed at by its predecessor and successor, while the nodes on the ends are pointed to
    /// by the list itself.
    pub fn push_front(&mut self, elem: T) {
        // new node needs +2 links, everything else should be +0
        let new_head = Node::new(elem);

        match self.head.take() {
            Some(old_head) => {
                // non-empty list, need to connect the old_head
                old_head.borrow_mut().prev = Some(new_head.clone()); // +1 new_head
                new_head.borrow_mut().next = Some(old_head); // +1 old_head
                self.head = Some(new_head); // +1 new_head, -1 old_head
                                            // total: +2 new_head, +0 old_head -- OK!
            }
            None => {
                // empty list, need to set the tail
                self.tail = Some(new_head.clone()); // +1 new_head
                self.head = Some(new_head); // +1 new_head
                                            // total: +2 new_head -- OK!
            }
        }
    }

    /// Same basic logic as push_front, but backwards.
    pub fn pop_front(&mut self) -> Option<T> {
        // need to take the old head, ensuring it's -2
        self.head.take().map(|old_head| {
            // -1 old
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // -1 new
                    // not emptying list
                    new_head.borrow_mut().prev.take(); // -1 old
                    self.head = Some(new_head); // +1 new
                                                // total: -2 old, +0 new
                }
                None => {
                    // emptying list
                    self.tail.take(); // -1 old
                                      // total: -2 old, (no new)
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
    }
}
