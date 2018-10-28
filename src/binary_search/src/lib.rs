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

/// Basic binary search
///
/// # Arguments
/// * `a` - A sorted slice that hold unique values
/// * `v` - The value to be searched
///
/// # Return
/// Index of the value
///
/// # Examples
///
/// ```
/// extern crate binary_search;
/// use binary_search::binary_search;
///
/// let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
/// assert_eq!(binary_search(&a, 9), Some(8));
/// assert_eq!(binary_search(&a, 10), None);
/// ```
pub fn binary_search<T: PartialOrd>(a: &[T], v: T) -> Option<usize> {
    let mut low = 0i32;
    let mut high = a.len() as i32 - 1;

    while low <= high {
        // prevent overflow
        let mid = low + ((high - low) >> 1);

        if a[mid as usize] == v {
            return Some(mid as usize);
        } else if a[mid as usize] < v {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}

/// Binary search the first element which equals to value
///
/// # Arguments
/// * `a` - A sorted slice that hold values
/// * `v` - The value to be searched
///
/// # Return
/// Index of the value
///
/// # Examples
///
/// ```
/// extern crate binary_search;
/// use binary_search::binary_search_first;
///
/// let a = vec![1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 8, 8, 9];
/// assert_eq!(binary_search_first(&a, 8), Some(10));
/// assert_eq!(binary_search_first(&a, 10), None);
/// ```
pub fn binary_search_first<T: PartialOrd>(a: &[T], v: T) -> Option<usize> {
    let mut low = 0i32;
    let mut high = a.len() as i32 - 1;

    while low <= high {
        // prevent overflow
        let mid = low + ((high - low) >> 1);

        if a[mid as usize] == v {
            if mid == 0 || a[mid as usize - 1] != v {
                return Some(mid as usize);
            } else {
                high = mid - 1;
            }
        } else if a[mid as usize] < v {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}

/// Binary search the last element which equals to value
///
/// # Arguments
/// * `a` - A sorted slice that hold values
/// * `v` - The value to be searched
///
/// # Return
/// Index of the value
///
/// # Examples
///
/// ```
/// extern crate binary_search;
/// use binary_search::binary_search_last;
///
/// let a = vec![1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 8, 8, 9];
/// assert_eq!(binary_search_last(&a, 8), Some(11));
/// assert_eq!(binary_search_last(&a, 10), None);
/// ```
pub fn binary_search_last<T: PartialOrd>(a: &[T], v: T) -> Option<usize> {
    let mut low = 0i32;
    let mut high = a.len() as i32 - 1;

    while low <= high {
        // prevent overflow
        let mid = low + ((high - low) >> 1);

        if a[mid as usize] == v {
            if mid == (a.len() as i32 - 1) || a[mid as usize + 1] != v {
                return Some(mid as usize);
            } else {
                low = mid + 1;
            }
        } else if a[mid as usize] < v {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}

/// Binary search the first element which greater than or equals to value
///
/// # Arguments
/// * `a` - A sorted slice that hold values
/// * `v` - The value to be searched
///
/// # Return
/// Index of the value
///
/// # Examples
///
/// ```
/// extern crate binary_search;
/// use binary_search::binary_search_first_gte;
///
/// let a = vec![1, 2, 2, 3, 5, 6, 6, 7, 9];
/// assert_eq!(binary_search_first_gte(&a, 0), Some(0));
/// assert_eq!(binary_search_first_gte(&a, 4), Some(4));
/// assert_eq!(binary_search_first_gte(&a, 8), Some(8));
/// assert_eq!(binary_search_first_gte(&a, 10), None);
/// ```
pub fn binary_search_first_gte<T: PartialOrd>(a: &[T], v: T) -> Option<usize> {
    let mut low = 0i32;
    let mut high = a.len() as i32 - 1;

    while low <= high {
        // prevent overflow
        let mid = low + ((high - low) >> 1);

        if a[mid as usize] >= v {
            if mid == 0 || a[mid as usize - 1] < v {
                return Some(mid as usize);
            } else {
                high = mid - 1;
            }
        } else {
            low = mid + 1;
        }
    }

    None
}

/// Binary search the last element which less than or equals to value
///
/// # Arguments
/// * `a` - A sorted slice that hold values
/// * `v` - The value to be searched
///
/// # Return
/// Index of the value
///
/// # Examples
///
/// ```
/// extern crate binary_search;
/// use binary_search::binary_search_last_lte;
///
/// let a = vec![1, 2, 2, 3, 5, 6, 6, 7, 9];
/// assert_eq!(binary_search_last_lte(&a, 0), None);
/// assert_eq!(binary_search_last_lte(&a, 4), Some(3));
/// assert_eq!(binary_search_last_lte(&a, 8), Some(7));
/// assert_eq!(binary_search_last_lte(&a, 10), Some(8));
/// ```
pub fn binary_search_last_lte<T: PartialOrd>(a: &[T], v: T) -> Option<usize> {
    let mut low = 0i32;
    let mut high = a.len() as i32 - 1;

    while low <= high {
        // prevent overflow
        let mid = low + ((high - low) >> 1);

        if a[mid as usize] <= v {
            if mid == (a.len() as i32 - 1) || a[mid as usize + 1] > v {
                return Some(mid as usize);
            } else {
                low = mid + 1;
            }
        } else {
            high = mid - 1;
        }
    }

    None
}

/// Binary search the element which equals to value in a sorted cycled slice
///
/// # Arguments
/// * `a` - A sorted cycled slice that hold unique values
/// * `v` - The value to be searched
///
/// # Return
/// Index of the value
///
/// # Examples
///
/// ```
/// extern crate binary_search;
/// use binary_search::binary_search_cycled;
///
/// let a = vec![5, 6, 7, 8, 9, 1, 2, 3, 4];
/// assert_eq!(binary_search_cycled(&a, 9), Some(4));
/// assert_eq!(binary_search_cycled(&a, 10), None);
/// ```
pub fn binary_search_cycled<T: PartialOrd>(a: &[T], v: T) -> Option<usize> {
    let mut low = 0i32;
    let mut high = a.len() as i32 - 1;

    while low <= high {
        // prevent overflow
        let mid = low + ((high - low) >> 1);
        let mid_val = &a[mid as usize];

        if *mid_val == v {
            return Some(mid as usize);
        } else if a[low as usize] <= *mid_val {
            // left side is ordered, right side is ordered or cycled
            if *mid_val > v && a[low as usize] <= v {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        } else {
            // left side is ordered or cycled, right side is ordered
            if *mid_val < v && a[high as usize] >= v {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(binary_search(&a, 0), None);
        assert_eq!(binary_search(&a, 1), Some(0));
        assert_eq!(binary_search(&a, 2), Some(1));
        assert_eq!(binary_search(&a, 3), Some(2));
        assert_eq!(binary_search(&a, 4), Some(3));
        assert_eq!(binary_search(&a, 5), Some(4));
        assert_eq!(binary_search(&a, 6), Some(5));
        assert_eq!(binary_search(&a, 7), Some(6));
        assert_eq!(binary_search(&a, 8), Some(7));
        assert_eq!(binary_search(&a, 9), Some(8));
        assert_eq!(binary_search(&a, 10), None);
    }

    #[test]
    fn test_binary_search_first() {
        let a = vec![1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 8, 8, 9];
        assert_eq!(binary_search_first(&a, 0), None);
        assert_eq!(binary_search_first(&a, 1), Some(0));
        assert_eq!(binary_search_first(&a, 2), Some(1));
        assert_eq!(binary_search_first(&a, 3), Some(3));
        assert_eq!(binary_search_first(&a, 4), Some(4));
        assert_eq!(binary_search_first(&a, 5), Some(6));
        assert_eq!(binary_search_first(&a, 6), Some(7));
        assert_eq!(binary_search_first(&a, 7), Some(9));
        assert_eq!(binary_search_first(&a, 8), Some(10));
        assert_eq!(binary_search_first(&a, 9), Some(12));
        assert_eq!(binary_search_first(&a, 10), None);
    }

    #[test]
    fn test_binary_search_last() {
        let a = vec![1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 8, 8, 9];
        assert_eq!(binary_search_last(&a, 0), None);
        assert_eq!(binary_search_last(&a, 1), Some(0));
        assert_eq!(binary_search_last(&a, 2), Some(2));
        assert_eq!(binary_search_last(&a, 3), Some(3));
        assert_eq!(binary_search_last(&a, 4), Some(5));
        assert_eq!(binary_search_last(&a, 5), Some(6));
        assert_eq!(binary_search_last(&a, 6), Some(8));
        assert_eq!(binary_search_last(&a, 7), Some(9));
        assert_eq!(binary_search_last(&a, 8), Some(11));
        assert_eq!(binary_search_last(&a, 9), Some(12));
        assert_eq!(binary_search_last(&a, 10), None);
    }

    #[test]
    fn test_binary_search_first_gte() {
        let a = vec![1, 2, 2, 3, 5, 6, 6, 7, 9];
        assert_eq!(binary_search_first_gte(&a, 0), Some(0));
        assert_eq!(binary_search_first_gte(&a, 1), Some(0));
        assert_eq!(binary_search_first_gte(&a, 2), Some(1));
        assert_eq!(binary_search_first_gte(&a, 3), Some(3));
        assert_eq!(binary_search_first_gte(&a, 4), Some(4));
        assert_eq!(binary_search_first_gte(&a, 5), Some(4));
        assert_eq!(binary_search_first_gte(&a, 6), Some(5));
        assert_eq!(binary_search_first_gte(&a, 7), Some(7));
        assert_eq!(binary_search_first_gte(&a, 8), Some(8));
        assert_eq!(binary_search_first_gte(&a, 9), Some(8));
        assert_eq!(binary_search_first_gte(&a, 10), None);
    }

    #[test]
    fn test_binary_search_last_lte() {
        let a = vec![1, 2, 2, 3, 5, 6, 6, 7, 9];
        assert_eq!(binary_search_last_lte(&a, 0), None);
        assert_eq!(binary_search_last_lte(&a, 1), Some(0));
        assert_eq!(binary_search_last_lte(&a, 2), Some(2));
        assert_eq!(binary_search_last_lte(&a, 3), Some(3));
        assert_eq!(binary_search_last_lte(&a, 4), Some(3));
        assert_eq!(binary_search_last_lte(&a, 5), Some(4));
        assert_eq!(binary_search_last_lte(&a, 6), Some(6));
        assert_eq!(binary_search_last_lte(&a, 7), Some(7));
        assert_eq!(binary_search_last_lte(&a, 8), Some(7));
        assert_eq!(binary_search_last_lte(&a, 9), Some(8));
        assert_eq!(binary_search_last_lte(&a, 10), Some(8));
    }

    #[test]
    fn test_binary_search_cycled_1() {
        let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(binary_search_cycled(&a, 0), None);
        assert_eq!(binary_search_cycled(&a, 1), Some(0));
        assert_eq!(binary_search_cycled(&a, 2), Some(1));
        assert_eq!(binary_search_cycled(&a, 3), Some(2));
        assert_eq!(binary_search_cycled(&a, 4), Some(3));
        assert_eq!(binary_search_cycled(&a, 5), Some(4));
        assert_eq!(binary_search_cycled(&a, 6), Some(5));
        assert_eq!(binary_search_cycled(&a, 7), Some(6));
        assert_eq!(binary_search_cycled(&a, 8), Some(7));
        assert_eq!(binary_search_cycled(&a, 9), Some(8));
        assert_eq!(binary_search_cycled(&a, 10), None);
    }

    #[test]
    fn test_binary_search_cycled_2() {
        let a = vec![5, 6, 7, 8, 9, 1, 2, 3, 4];
        assert_eq!(binary_search_cycled(&a, 0), None);
        assert_eq!(binary_search_cycled(&a, 1), Some(5));
        assert_eq!(binary_search_cycled(&a, 2), Some(6));
        assert_eq!(binary_search_cycled(&a, 3), Some(7));
        assert_eq!(binary_search_cycled(&a, 4), Some(8));
        assert_eq!(binary_search_cycled(&a, 5), Some(0));
        assert_eq!(binary_search_cycled(&a, 6), Some(1));
        assert_eq!(binary_search_cycled(&a, 7), Some(2));
        assert_eq!(binary_search_cycled(&a, 8), Some(3));
        assert_eq!(binary_search_cycled(&a, 9), Some(4));
        assert_eq!(binary_search_cycled(&a, 10), None);
    }

    #[test]
    fn test_binary_search_cycled_3() {
        let a = vec![9, 1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(binary_search_cycled(&a, 0), None);
        assert_eq!(binary_search_cycled(&a, 1), Some(1));
        assert_eq!(binary_search_cycled(&a, 2), Some(2));
        assert_eq!(binary_search_cycled(&a, 3), Some(3));
        assert_eq!(binary_search_cycled(&a, 4), Some(4));
        assert_eq!(binary_search_cycled(&a, 5), Some(5));
        assert_eq!(binary_search_cycled(&a, 6), Some(6));
        assert_eq!(binary_search_cycled(&a, 7), Some(7));
        assert_eq!(binary_search_cycled(&a, 8), Some(8));
        assert_eq!(binary_search_cycled(&a, 9), Some(0));
        assert_eq!(binary_search_cycled(&a, 10), None);
    }

    #[test]
    fn test_binary_search_cycled_4() {
        let a = vec![2, 3, 4, 5, 6, 7, 8, 9, 1];
        assert_eq!(binary_search_cycled(&a, 0), None);
        assert_eq!(binary_search_cycled(&a, 1), Some(8));
        assert_eq!(binary_search_cycled(&a, 2), Some(0));
        assert_eq!(binary_search_cycled(&a, 3), Some(1));
        assert_eq!(binary_search_cycled(&a, 4), Some(2));
        assert_eq!(binary_search_cycled(&a, 5), Some(3));
        assert_eq!(binary_search_cycled(&a, 6), Some(4));
        assert_eq!(binary_search_cycled(&a, 7), Some(5));
        assert_eq!(binary_search_cycled(&a, 8), Some(6));
        assert_eq!(binary_search_cycled(&a, 9), Some(7));
        assert_eq!(binary_search_cycled(&a, 10), None);
    }
}
