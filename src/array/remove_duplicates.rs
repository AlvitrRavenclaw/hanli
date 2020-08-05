struct Solution;

impl Solution {
    pub fn remove_duplicates(_nums: &mut Vec<i32>) -> i32 {
        0
    }
}

#[test]
fn test_remove_duplicates() {
    let v = &mut vec![1, 2, 3];
    assert_eq!(Solution::remove_duplicates(v), 0)
}
