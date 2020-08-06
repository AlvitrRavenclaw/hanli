#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut ret = 0;
        if prices.len() == 0 {
            return ret;
        }
        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                ret += prices[i] - prices[i - 1]
            }
        }
        return ret;
    }
}

#[test]
fn test_max_profit() {
    let case0: Vec<i32> = vec![7, 1, 5, 3, 6, 4];
    assert_eq!(Solution::max_profit(case0), 7);

    let case1: Vec<i32> = vec![1, 2, 3, 4, 5];
    assert_eq!(Solution::max_profit(case1), 4);

    let case2: Vec<i32> = vec![7, 6, 4, 3, 1];
    assert_eq!(Solution::max_profit(case2), 0);
}