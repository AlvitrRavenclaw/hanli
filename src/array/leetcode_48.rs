use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        for i in 0..matrix.len() {
            for j in 0..i + 1 {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
        for row in matrix.iter_mut() {
            row.reverse()
        }
    }
}

#[test]
fn test_rotate() {
    let mut case0 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(&mut case0);
    println!("{:?}", case0);

    let mut case1 = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    Solution::rotate(&mut case1);
    println!("{:?}", case1);
}
