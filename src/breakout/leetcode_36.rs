#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut visit = [[0u8; 9]; 9];

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    continue;
                }
                let val = board[i][j] as usize - 49usize;
                if visit[val][i] & 1u8 != 0 {
                    return false;
                } else {
                    visit[val][i] |= 1u8;
                }
                if visit[val][j] & 2u8 != 0 {
                    return false;
                } else {
                    visit[val][j] |= 2u8;
                }
                let p = (i / 3) as usize;
                let q = (j / 3) as usize;
                let k: usize = p * 3 + q;
                if visit[val][k] &4u8 != 0 {
                    return false;
                } else {
                    visit[val][k] |= 4u8;
                }
            }
        }
        
        return true;
    }
}

#[test]
fn test_is_valid_sudoku() {
    let board: Vec<Vec<char>> = vec![
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
    let solution = Solution::is_valid_sudoku(board);
    println!("{}", solution);
}
