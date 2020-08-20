#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_flags = vec![[false; 9]; 9];
        let mut col_flags = vec![[false; 9]; 9];
        let mut blk_flags = vec![[false; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    continue;
                }
                let v = (board[i][j] as u8 - 49) as usize;
                if row_flags[v][i] {
                    return false;
                } else {
                    row_flags[v][i] = true
                }
                if col_flags[v][j] {
                    return false;
                } else {
                    col_flags[v][j] = true
                }
                let _i = (i / 3) as usize;
                let _j = (j / 3) as usize;
                let k = _i * 3 + _j;
                if blk_flags[v][k] {
                    return false;
                } else {
                    blk_flags[v][k] = true;
                }
            }
        }
        return true;
    }
}

#[test]
fn test_is_valid_sudoku() {
    let case0 = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    println!("{}", Solution::is_valid_sudoku(case0));

    let case1 = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    println!("{}", Solution::is_valid_sudoku(case1));
}
