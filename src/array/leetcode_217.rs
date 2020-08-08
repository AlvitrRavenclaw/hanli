use std::collections::HashSet;

#[allow(dead_code)] 
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set: HashSet = HashSet::new<i32>();
        for i in 0..nums.len() {
            if set.contains(nums[i]) {
                return true
            }
            set.insert(nums[i])
        }
        return false
    }
}

#[test]
fn test_contains_duplicate() {
    let mut case0 = vec![1, 2, 3, 1];
    assert_eq!(Solution::contains_duplicate(case0), true);

    let mut case1 = vec![1, 2, 3, 4];
    assert_eq!(Solution::contains_duplicate(case1), false);

    let mut case2 = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
    assert_eq!(Solution::contains_duplicate(case2), true);
}

