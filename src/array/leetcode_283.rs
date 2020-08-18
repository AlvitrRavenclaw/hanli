#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // for i in 0..nums.len() {
        //     if nums[i] != 0 {
        //         continue
        //     }
        //     for j in i+1..nums.len() {
        //         if nums[j] != 0 {
        //             nums[i] = nums[j];
        //             nums[j] = 0;
        //             break;
        //         }
        //     }
        // }
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != 0 {
                if i != j {
                    nums.swap(i, j);
                }
                i += 1;
            }
        }
    }
}

#[test]
fn test_move_zeroes() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    println!("{:?}", nums);
}
