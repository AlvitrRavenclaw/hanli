#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut i: usize = 0;
        for j in 1..nums.len() {
            if nums[i] != nums[j] {
                if j > i + 1 {
                    nums[i + 1] = nums[j];
                }
                i += 1;
            }
        }
        return (i + 1) as i32;
    }
}

#[test]
fn test_remove_duplicates() {
    let mut case0 = vec![1, 1, 2];
    let ret0 = Solution::remove_duplicates(&mut case0);
    println!("{:?}", case0);
    assert_eq!(ret0, 2);

    let mut case1 = vec![0, 0, 1, 1, 2, 3, 3, 4];
    let ret1 = Solution::remove_duplicates(&mut case1);
    println!("{:?}", case1);
    assert_eq!(ret1, 5);

    let mut case2 = vec![];
    let ret2 = Solution::remove_duplicates(&mut case2);
    println!("{:?}", case2);
    assert_eq!(ret2, 0);
}
