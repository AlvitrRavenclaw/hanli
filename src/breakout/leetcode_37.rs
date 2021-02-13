#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Solution::solve(board);
    }

    pub fn solve(board: &mut Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    continue
                }
                for ch in '1'..='9' {
                    if !Solution::is_valid(board, i, j, ch) {
                        continue;
                    }
                    board[i][j] = ch;
                    if Solution::solve(board) {
                        return true;
                    } else {
                        board[i][j] = '.';
                    }
                }
                return false;
            }
        }
        return true;
    }
    
    pub fn is_valid(board: &mut Vec<Vec<char>>, i: usize, j: usize, ch: char) -> bool {
        for k in 0..9 {
            if board[i][k] != '.' && board[i][k] == ch {
                return false;
            }
            if board[k][j] != '.' && board[k][j] == ch {
                return false;
            }
            let p = (i / 3) as usize;
            let q = (j / 3) as usize;
            let _i = 3 * p + k / 3;
            let _j = 3 * q + k % 3;
            if board[_i][_j] != '.' && board[_i][_j] == ch {
                return false;
            }
        }
        return true;
    }
}

#[test]
fn test_solve_sudoku() {
    let mut board: Vec<Vec<char>> = vec![
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
    Solution::solve_sudoku(&mut board);

    for row in board.iter() {
        println!("{:?}", row);
    }
}
