use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, x) in nums.iter().enumerate() {
            if map.contains_key(x) {
                return vec![*map.get(x).unwrap(), i as i32];
            } else {
                map.insert(target - x, i as i32);
            }
        }
        return vec![-1, -1];
    }
}

#[test]
fn test_two_sum() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
}
