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

pub trait List<T> {
    fn size(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    #[inline]
    fn non_empty(&self) -> bool {
        self.size() != 0
    }

    fn push_back(&mut self, elem: T);

    fn push_front(&mut self, elem: T);

    fn pop_back(&mut self) -> Option<T>;

    fn pop_front(&mut self) -> Option<T>;

    fn clear(&mut self);
}

pub mod linked_list;
