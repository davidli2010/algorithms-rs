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

use std::collections::HashMap;

pub fn two_sum(nums: &[i32], target: i32) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut map = HashMap::new();

    for i in 0..nums.len() {
        let diff = target - nums[i];
        if map.contains_key(&diff) {
            let idx = map.get(&diff).unwrap();
            result.push((*idx, i));
        } else {
            map.insert(nums[i], i);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(&[2, 7, 11, 15], 9), vec![(0usize, 1usize)]);
        assert_eq!(two_sum(&[2, 7, 11, 15], 13), vec![(0usize, 2usize)]);
        assert_eq!(two_sum(&[2, 7, 11, 15], 18), vec![(1usize, 2usize)]);
        assert_eq!(
            two_sum(&[2, 7, 4, 5], 9),
            vec![(0usize, 1usize), (2usize, 3usize)]
        );
    }
}
