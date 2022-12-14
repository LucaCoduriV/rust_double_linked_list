pub mod unsafe_linked_list;

use std::cell::{RefCell};
use std::fmt::{Debug, Display, Formatter};
use std::rc::{Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    next: Link<T>,
    prev: Link<T>,
    data: T,
}

impl<T> Node<T> {
    pub fn new(data: T, next: Link<T>, prev: Link<T>) -> Self {
        Self { data, prev, next }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, tail: None, len: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_back(&mut self, el: T) {
        let new_node = Rc::new(RefCell::new(Node::new(el, None, None)));
        if let Some(previous_tail_node) = self.tail.take() {
            previous_tail_node.as_ref().borrow_mut().next = Some(new_node.clone());
            new_node.as_ref().borrow_mut().prev = Some(previous_tail_node.clone());
            self.tail = Some(new_node);
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        }
        self.len += 1
    }

    pub fn push_front(&mut self, el: T) {
        let new_node = Rc::new(RefCell::new(Node::new(el, None, None)));
        if let Some(previous_head_node) = self.head.take() {
            previous_head_node.as_ref().borrow_mut().prev = Some(new_node.clone());
            new_node.as_ref().borrow_mut().next = Some(previous_head_node);
            self.head = Some(new_node)
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        }
        self.len += 1
    }

    pub fn pop_back(&mut self) -> Option<T> {
        return if let Some(last_node) = self.tail.take() {
            if let Some(before_last_node) = last_node.as_ref().borrow_mut().prev.take() {
                before_last_node.as_ref().borrow_mut().next = None;
                self.tail = Some(before_last_node.clone());
            } else {
                self.head.take();
            }
            self.len -= 1;
            Some(Rc::try_unwrap(last_node).ok().unwrap().into_inner().data)
        } else {
            None
        };
    }

    pub fn pop_front(&mut self) -> Option<T> {
        return if let Some(first_node) = self.head.take() {
            if let Some(after_first_node) = first_node.as_ref().borrow_mut().next.take() {
                after_first_node.as_ref().borrow_mut().prev = None;
                self.head = Some(after_first_node.clone());
            } else {
                self.tail.take();
            }
            self.len -= 1;
            Some(Rc::try_unwrap(first_node).ok().unwrap().into_inner().data)
        } else {
            None
        };
    }
}

// Ne pas oublier de drop chaque Rc pour ??viter les cycling ref
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(node) = self.tail.take() {
            node.as_ref().borrow_mut().next.take();
            self.tail = node.as_ref().borrow_mut().prev.take();
        }
        self.head.take();
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return write!(f, "[]");
        }

        let mut node_ref = self.head.as_ref().unwrap().clone();
        write!(f, "[{}", node_ref.as_ref().borrow().data)?;
        loop {
            if let Some(ref next) = node_ref.clone().as_ref().borrow_mut().next {
                write!(f, ",")?;
                node_ref = next.clone();
                write!(f, "{}", node_ref.as_ref().borrow().data)?;
            } else {
                break;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = <LinkedListIterator<T> as Iterator>::Item;
    type IntoIter = LinkedListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIterator::new(self)
    }
}

pub struct LinkedListIterator<T> {
    list: LinkedList<T>,
}

impl<T> LinkedListIterator<T> {
    fn new(list: LinkedList<T>) -> Self {
        Self { list }
    }
}

impl<T> Iterator for LinkedListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> DoubleEndedIterator for LinkedListIterator<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}