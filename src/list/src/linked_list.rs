// Copyright 2018 David Li
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::List;
use std::mem;
use std::ptr;

// reference:
//     https://github.com/marchelzo/doubly
//     https://github.com/contain-rs/linked-list

#[derive(Debug)]
struct RawPtr<T> {
    ptr: *const Node<T>,
}

impl<T> RawPtr<T> {
    #[inline]
    fn none() -> Self {
        RawPtr { ptr: ptr::null() }
    }

    #[inline]
    fn some(ptr: &Node<T>) -> Self {
        RawPtr { ptr }
    }

    #[inline]
    fn as_mut(&mut self) -> Option<&mut Node<T>> {
        if self.ptr.is_null() {
            None
        } else {
            unsafe { Some(&mut *(self.ptr as *mut Node<T>)) }
        }
    }

    #[inline]
    fn take(&mut self) -> Self {
        mem::replace(self, RawPtr::none())
    }

    #[inline]
    fn replace(&mut self, dest: Self) -> Self {
        mem::replace(self, dest)
    }
}

struct Node<T> {
    prev: RawPtr<T>,
    next: Option<Box<Node<T>>>,
    elem: T,
}

impl<T> Node<T> {
    #[inline]
    fn new(elem: T) -> Node<T> {
        Node {
            prev: RawPtr::none(),
            next: None,
            elem,
        }
    }

    #[inline]
    fn link(&mut self, mut next: Box<Self>) {
        next.prev = RawPtr::some(self);
        self.next = Some(next);
    }
}

pub struct LinkedList<T> {
    size: usize,
    head: Option<Box<Node<T>>>,
    tail: RawPtr<T>,
}

impl<T> LinkedList<T> {
    #[inline]
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: RawPtr::none(),
            size: 0,
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|head| head.as_ref()),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_mut().map(|head| head.as_mut()),
        }
    }
}

impl<T> List<T> for LinkedList<T> {
    #[inline]
    fn size(&self) -> usize {
        self.size
    }

    fn push_back(&mut self, elem: T) {
        let node = Box::new(Node::new(elem));
        let mut old_tail = self.tail.replace(RawPtr::some(&*node));
        match old_tail.as_mut() {
            None => self.head = Some(node),
            Some(tail) => tail.link(node),
        }
        self.size += 1;
    }

    fn push_front(&mut self, elem: T) {
        let mut node = Box::new(Node::new(elem));
        match self.head.take() {
            None => self.tail = RawPtr::some(&*node),
            Some(head) => node.link(head),
        }
        self.head = Some(node);
        self.size += 1;
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().as_mut().and_then(|node| {
            self.size -= 1;
            match node.prev.as_mut() {
                None => self.head.take().map(|node| node.elem),
                Some(prev) => {
                    self.tail = RawPtr::some(prev);
                    prev.next.take().map(|node| node.elem)
                }
            }
        })
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut head| {
            self.size -= 1;
            match head.next.take() {
                None => self.tail = RawPtr::none(),
                Some(next) => self.head = Some(next),
            }
            head.elem
        })
    }

    #[inline]
    fn clear(&mut self) {
        while self.non_empty() {
            self.pop_front();
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.next.take().map(|node| {
            match node.next.as_ref() {
                None => self.next = None,
                Some(next) => self.next = Some(&next),
            };
            &node.elem
        })
    }
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        self.next.take().map(|node| {
            match node.next.as_mut() {
                None => self.next = None,
                Some(next) => self.next = Some(next),
            };
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut list = LinkedList::new();
        assert_eq!(list.size(), 0);
        assert_eq!(list.is_empty(), true);
        assert_eq!(list.non_empty(), false);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);

        // 40 20 10 30 50
        list.push_back(10);
        assert_eq!(list.size(), 1);
        list.push_front(20);
        assert_eq!(list.size(), 2);
        list.push_back(30);
        assert_eq!(list.size(), 3);
        list.push_front(40);
        assert_eq!(list.size(), 4);
        list.push_back(50);
        assert_eq!(list.size(), 5);
        assert_eq!(list.is_empty(), false);
        assert_eq!(list.non_empty(), true);

        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.size(), 4);
        assert_eq!(list.pop_back(), Some(50));
        assert_eq!(list.size(), 3);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.size(), 2);
        assert_eq!(list.pop_back(), Some(30));
        assert_eq!(list.size(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.size(), 0);
        assert_eq!(list.is_empty(), true);
        assert_eq!(list.non_empty(), false);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = LinkedList::new();
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);
        list.push_back(40);
        list.push_back(50);

        for (i, v) in list.iter().enumerate() {
            assert_eq!(*v, 10 * (i + 1) as i32);
        }

        assert_eq!(5, list.iter().count());
    }

    #[test]
    fn test_iter_mut() {
        let mut list = LinkedList::new();
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);
        list.push_back(40);
        list.push_back(50);

        for (i, v) in list.iter_mut().enumerate() {
            assert_eq!(*v, 10 * (i as i32 + 1));
        }

        assert_eq!(5, list.iter().count());

        for i in list.iter_mut() {
            *i += 5;
        }

        for (i, v) in list.iter_mut().enumerate() {
            assert_eq!(*v, 10 * (i as i32 + 1) + 5);
        }
    }
}
