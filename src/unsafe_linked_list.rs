use std::ptr::null_mut;

struct Node<T> {
    next: *mut Node<T>,
    prev: *mut Node<T>,
    data: T,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self { data, prev: null_mut(), next: null_mut() }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: null_mut(), tail: null_mut(), len: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_back(&mut self, el: T) {
        let new_node = Box::into_raw(Box::new(Node::new(el)));

        if self.tail.is_null() {
            self.tail = new_node;
            self.head = new_node;
        } else {
            unsafe {
                (*self.tail).next = new_node;
                (*new_node).prev = self.tail;
                self.tail = new_node;
            }
        }
        self.len += 1;
    }

    pub fn push_front(&mut self, el:T){
        let new_node = Box::into_raw(Box::new(Node::new(el)));

        if self.head.is_null() {
            self.head = new_node;
            self.tail = new_node;
        } else {
            unsafe {
                (*self.head).prev = new_node;
                (*new_node).next = self.head;
                self.head = new_node;
            }
        }
        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        }

        let old_tail = self.tail;
        unsafe {
            self.tail = (*self.tail).prev;
            if self.tail.is_null() {
                self.head = null_mut();
            } else {
                (*self.tail).next = null_mut();
            }
        }
        self.len -= 1;
        unsafe { Some(Box::from_raw(old_tail).data) }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }

        let old_head = self.head;
        unsafe {
            self.head = (*self.head).next;
            if self.head.is_null() {
                self.tail = null_mut();
            } else {
                (*self.head).prev = null_mut();
            }
        }
        self.len -= 1;
        unsafe { Some(Box::from_raw(old_head).data) }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { current: self.head, _marker: std::marker::PhantomData }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { current: self.head, _marker: std::marker::PhantomData }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head;
        while !current.is_null() {
            let next = unsafe { (*current).next };
            unsafe { Box::from_raw(current) };
            current = next;
        }
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

pub struct Iter<'a, T> {
    current: *mut Node<T>,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }

        unsafe {
            let data = &(*self.current).data;
            self.current = (*self.current).next;
            Some(data)
        }
    }
}

pub struct IterMut<'a, T> {
    current: *mut Node<T>,
    _marker: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }

        unsafe {
            let data = &mut (*self.current).data;
            self.current = (*self.current).next;
            Some(data)
        }
    }
}
