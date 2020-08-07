#[allow(dead_code)]
struct Solution;

pub fn reverse(nums: &mut Vec<i32>, i: usize, j: usize) {
    let mut _i = i;
    let mut _j = j;
    while _i < _j {
        let tmp = nums[_i];
        nums[_i] = nums[_j];
        nums[_j] = tmp;
        _i += 1;
        _j -= 1;
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let _k = k % (nums.len() as i32);
        if _k == 0 {
            return
        }
        reverse(nums, 0, nums.len() - 1);
        reverse(nums, 0, (_k - 1) as usize);
        reverse(nums, _k as usize, nums.len() - 1);
    }
}

#[test]
fn test_max_profit() {
    let mut case0: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut case0, 3);
    println!("{:?}", case0);

    let mut case1: Vec<i32> = vec![-1, -100, 3, 99];
    Solution::rotate(&mut case1, 2);
    println!("{:?}", case1)
}