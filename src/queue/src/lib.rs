extern crate list;

use list::linked_list::{self, LinkedList};
use list::List;

pub trait Queue<T> {
    fn enqueue(&mut self, item: T);

    fn dequeue(&mut self) -> Option<T>;

    fn size(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

pub struct LinkedQueue<T> {
    list: LinkedList<T>,
}

impl<T> LinkedQueue<T> {
    pub fn new() -> LinkedQueue<T> {
        LinkedQueue {
            list: LinkedList::new(),
        }
    }

    pub fn iter(&self) -> LinkedIter<T> {
        LinkedIter {
            iter: self.list.iter(),
        }
    }
}

impl<T> Queue<T> for LinkedQueue<T> {
    fn enqueue(&mut self, item: T) {
        self.list.push_back(item)
    }

    fn dequeue(&mut self) -> Option<T> {
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
    fn test_linked_queue() {
        let mut queue = LinkedQueue::new();
        assert_eq!(queue.size(), 0);
        assert_eq!(queue.is_empty(), true);

        queue.enqueue(10);
        assert_eq!(queue.size(), 1);
        assert_eq!(queue.is_empty(), false);

        queue.enqueue(20);
        assert_eq!(queue.size(), 2);
        assert_eq!(queue.is_empty(), false);

        queue.enqueue(30);
        assert_eq!(queue.size(), 3);
        assert_eq!(queue.is_empty(), false);

        queue.enqueue(40);
        assert_eq!(queue.size(), 4);
        assert_eq!(queue.is_empty(), false);

        queue.enqueue(50);
        assert_eq!(queue.size(), 5);
        assert_eq!(queue.is_empty(), false);

        assert_eq!(queue.dequeue(), Some(10));
        assert_eq!(queue.size(), 4);
        assert_eq!(queue.is_empty(), false);

        assert_eq!(queue.dequeue(), Some(20));
        assert_eq!(queue.size(), 3);
        assert_eq!(queue.is_empty(), false);

        assert_eq!(queue.dequeue(), Some(30));
        assert_eq!(queue.size(), 2);
        assert_eq!(queue.is_empty(), false);

        assert_eq!(queue.dequeue(), Some(40));
        assert_eq!(queue.size(), 1);
        assert_eq!(queue.is_empty(), false);

        assert_eq!(queue.dequeue(), Some(50));
        assert_eq!(queue.size(), 0);
        assert_eq!(queue.is_empty(), true);

        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.size(), 0);
        assert_eq!(queue.is_empty(), true);
    }

    #[test]
    fn test_linked_queue_iter() {
        let mut queue = LinkedQueue::new();
        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);
        queue.enqueue(40);
        queue.enqueue(50);
        assert_eq!(queue.iter().count(), 5);

        for (i, v) in queue.iter().enumerate() {
            assert_eq!(*v, 10 * (i as i32 + 1));
        }
    }
}
