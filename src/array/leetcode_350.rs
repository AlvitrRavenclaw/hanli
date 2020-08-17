use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        for x in nums1 {
            let count: &mut i32 = map.entry(x).or_insert(0);
            *count += 1;
        }
        let mut ret = vec![];
        for x in nums2 {
            if !map.contains_key(&x) {
                continue;
            }
            ret.push(x);

            let cnt = map.get(&x).unwrap() - 1;
            if cnt == 0 {
                map.remove(&x);
            } else {
                map.insert(x, cnt);
            }
        }
        return ret;
    }
}

#[test]
fn test_intersect() {
    println!("{:?}", Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]));
    println!("{:?}", Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]));
}