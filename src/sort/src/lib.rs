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

pub fn bubble_sort<T: PartialOrd + Copy>(a: &mut [T]) {
    let n = a.len();

    if n <= 1 {
        return;
    }

    for i in 0..n {
        let mut swapped = false;

        for j in 0..(n - i - 1) {
            if a[j] > a[j + 1] {
                a.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

pub fn insertion_sort<T: PartialOrd + Copy>(a: &mut [T]) {
    let n = a.len();

    if n <= 1 {
        return;
    }

    for i in 1..n {
        let x = a[i];
        let mut j = i as i32 - 1;
        while j >= 0 {
            if a[j as usize] > x {
                a[j as usize + 1] = a[j as usize];
                j = j - 1;
            } else {
                break;
            }
        }
        a[(j + 1) as usize] = x;
    }
}

pub fn selection_sort<T: PartialOrd + Copy>(a: &mut [T]) {
    let n = a.len();

    if n <= 1 {
        return;
    }

    for i in 0..n {
        let mut min = i;
        for j in i..n {
            if a[j] < a[min] {
                min = j;
            }
        }

        if min != i {
            a.swap(min, i);
        }
    }
}

pub fn quick_sort<T: PartialOrd + Copy>(a: &mut [T]) {
    quick_sort_part(a)
}

fn quick_sort_part<T: PartialOrd + Copy>(a: &mut [T]) {
    let n = a.len();

    if n <= 1 {
        return;
    }

    let p = partition(a);
    quick_sort_part(&mut a[0..p]);
    quick_sort_part(&mut a[p + 1..n]);
}

fn partition<T: PartialOrd + Copy>(a: &mut [T]) -> usize {
    let n = a.len();
    let pivot = a[n - 1];

    let mut i = 0;

    for j in 0..(n - 1) {
        if a[j] < pivot {
            a.swap(i, j);
            i += 1;
        }
    }

    a.swap(i, n - 1);

    return i;
}

pub fn smallest_n<T: PartialOrd + Copy>(a: &mut [T], n: usize) -> Option<T> {
    let len = a.len();

    if len < n || n == 0 {
        return None;
    }

    let p = partition(a);
    if p == n - 1 {
        Some(a[p])
    } else if p < n - 1 {
        smallest_n(&mut a[p + 1..len], n - p - 1)
    } else {
        smallest_n(&mut a[0..p], n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4, 3, 5, 1, 2];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut v = vec![5, 4, 3, 2, 1];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut v = vec![4, 3, 5, 1, 2];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut v = vec![5, 4, 3, 2, 1];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_selection_sort() {
        let mut v = vec![4, 3, 5, 1, 2];
        selection_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut v = vec![5, 4, 3, 2, 1];
        selection_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![4, 3, 5, 1, 2];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut v = vec![5, 4, 3, 2, 1];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_smallest_n() {
        let mut v = vec![4, 3, 5, 1, 2];
        assert_eq!(smallest_n(&mut v, 0), None);
        assert_eq!(smallest_n(&mut v, 1), Some(1));
        assert_eq!(smallest_n(&mut v, 2), Some(2));
        assert_eq!(smallest_n(&mut v, 3), Some(3));
        assert_eq!(smallest_n(&mut v, 4), Some(4));
        assert_eq!(smallest_n(&mut v, 5), Some(5));
        assert_eq!(smallest_n(&mut v, 6), None);
    }
}
