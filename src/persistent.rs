//! # Persistent immutable singly-linked list
//!
//! Exactly the list that functional programmers have come to know and love.
//! Basically, we can get the head or the tail and put someone's head on someone else's tail. The power of immutability.
//!
//! The most important thing about a persistent list is that you can manipulate the tails of lists basically for free.
//! For instance, this isn't an uncommon workload to see with a persistent list:
//!
//! list1 = A -> B -> C -> D
//! list2 = tail(list1) = B -> C -> D
//! list3 = push(list2, X) = X -> B -> C -> D
//!
//! But at the end we want the memory to look like this:
//!
//! list1 -> A ---+
//!               |
//!               v
//! list2 ------> B -> C -> D
//!               ^
//!               |
//! list3 -> X ---+
//!
//! This just can't work with Boxes, because ownership of B is shared. Who should free it? If I drop list2, does it free B? With boxes we certainly would expect so!
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    /// Constructor create and returns a new List.
    pub fn new() -> Self {
        List { head: None }
    }

    /// Append method takes a list and an element, and returns a List.
    pub fn append(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }

    /// The logical inverse of append operation. It takes a list and returns
    /// the whole list with the first element removed. All that is is cloning
    /// the second element in the list (if it exists).
    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    /// Returns a reference to the first element
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(1).append(2).append(3).append(4);
        assert_eq!(list.head(), Some(&4));

        let list = list.tail();
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }
}
