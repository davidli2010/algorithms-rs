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

extern crate list;

use list::linked_list;
use list::List;

pub trait Bag<T> {
    fn add(&mut self, item: T);

    fn size(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

pub struct LinkedBag<T> {
    list: linked_list::LinkedList<T>,
}

impl<T> LinkedBag<T> {
    pub fn new() -> LinkedBag<T> {
        LinkedBag {
            list: linked_list::LinkedList::new(),
        }
    }

    pub fn iter(&self) -> LinkedIter<T> {
        LinkedIter {
            iter: self.list.iter(),
        }
    }
}

impl<T> Bag<T> for LinkedBag<T> {
    #[inline]
    fn add(&mut self, item: T) {
        self.list.push_back(item)
    }

    #[inline]
    fn size(&self) -> usize {
        self.list.size()
    }
}

pub struct LinkedIter<'a, T: 'a> {
    iter: list::linked_list::Iter<'a, T>,
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
    fn test_linked_bag() {
        let mut b = LinkedBag::new();
        assert_eq!(b.size(), 0);
        assert_eq!(b.is_empty(), true);

        b.add(10);
        assert_eq!(b.size(), 1);
        assert_eq!(b.is_empty(), false);

        b.add(20);
        assert_eq!(b.size(), 2);
        assert_eq!(b.is_empty(), false);
    }

    #[test]
    fn test_linked_bag_iter() {
        let mut b = LinkedBag::new();
        b.add(10);
        b.add(20);
        b.add(30);
        b.add(40);
        b.add(50);

        assert_eq!(5, b.size());

        for (i, v) in b.iter().enumerate() {
            assert_eq!(*v, 10 * (i as i32 + 1));
        }
    }
}
