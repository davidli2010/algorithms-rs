extern crate list;

use list::linked_list::{self, LinkedList};
use list::List;

pub trait Stack<T> {
    fn push(&mut self, item: T);

    fn pop(&mut self) -> Option<T>;

    fn size(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

pub struct LinkedStack<T> {
    list: LinkedList<T>,
}

impl<T> LinkedStack<T> {
    pub fn new() -> LinkedStack<T> {
        LinkedStack {
            list: LinkedList::new(),
        }
    }

    pub fn iter(&self) -> LinkedIter<T> {
        LinkedIter {
            iter: self.list.iter(),
        }
    }
}

impl<T> Stack<T> for LinkedStack<T> {
    #[inline]
    fn push(&mut self, item: T) {
        self.list.push_front(item)
    }

    #[inline]
    fn pop(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    #[inline]
    fn size(&self) -> usize {
        self.list.size()
    }
}

pub struct LinkedIter<'a, T: 'a> {
    iter: linked_list::Iter<'a, T>,
}

impl<'a, T> Iterator for LinkedIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_stack() {
        let mut stack = LinkedStack::new();
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.is_empty(), true);

        stack.push(10);
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.is_empty(), false);

        stack.push(20);
        assert_eq!(stack.size(), 2);
        assert_eq!(stack.is_empty(), false);

        stack.push(30);
        assert_eq!(stack.size(), 3);
        assert_eq!(stack.is_empty(), false);

        stack.push(40);
        assert_eq!(stack.size(), 4);
        assert_eq!(stack.is_empty(), false);

        stack.push(50);
        assert_eq!(stack.size(), 5);
        assert_eq!(stack.is_empty(), false);

        assert_eq!(stack.pop(), Some(50));
        assert_eq!(stack.size(), 4);
        assert_eq!(stack.is_empty(), false);

        assert_eq!(stack.pop(), Some(40));
        assert_eq!(stack.size(), 3);
        assert_eq!(stack.is_empty(), false);

        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.size(), 2);
        assert_eq!(stack.is_empty(), false);

        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.is_empty(), false);

        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.is_empty(), true);

        assert_eq!(stack.pop(), None);
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.is_empty(), true);
    }

    #[test]
    fn test_linked_stack_iter() {
        let mut stack = LinkedStack::new();
        assert_eq!(stack.iter().count(), 0);

        stack.push(10);
        stack.push(20);
        stack.push(30);
        stack.push(40);
        stack.push(50);

        assert_eq!(stack.iter().count(), 5);

        for (i, v) in stack.iter().enumerate() {
            assert_eq!(*v, 10 * (5 - i as i32));
        }
    }
}
