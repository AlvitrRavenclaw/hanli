#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![];
        let mut carry = 1;
        for x in digits.iter().rev() {
            let tmp = *x + carry;
            ret.push(tmp % 10);
            carry = if tmp < 10 {0} else {1};
        }
        if carry == 1 {
            ret.push(carry)
        }
        ret.reverse();
        return ret;
    }
}

#[test]
fn test_plus_one() {
    println!("{:?}", Solution::plus_one(vec![9, 9]));
    println!("{:?}", Solution::plus_one(vec![1, 2, 3]));
}