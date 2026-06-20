// @leet start
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        struct Context {
            result: bool,
            path: Vec<char>,
            num_letters: isize,
        }

        let mut context = Context { result: false, path: Vec::new(), num_letters: word.len() as isize };

        fn get_valid_neighbors(cell: (isize, isize), index: usize, board: &Vec<Vec<char>>, word: &String) -> Vec<(isize, isize)> {
            let mut valid_neighbors = Vec::new();
            let num_rows = board.len() as isize;
            let num_cols = board[0].len() as isize;
            let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

            for dir in dirs {
                let test_cell = (cell.0 + dir.0, cell.1 + dir.1);

                if test_cell.0 >= 0 && test_cell.1 >= 0 && test_cell.0 < num_rows && test_cell.1 < num_cols {
                    let found_char = board[test_cell.0 as usize][test_cell.1 as usize];
                    if found_char == word[index] {
                        valid_neighbors.push(test_cell.clone());
                    }
                }
            }

            valid_neighbors
        }

        fn dfs(index: usize, board: &Vec<Vec<char>>, word: &String, context: &mut Context) {
            // base case more common
            if context.result == true {
                return;
            }

            // base case found solution yay good times USA
            if context.path.len() as isize == context.num_letters {
                context.result == true;
                return;
            }
        }

        let shit = get_valid_neighbors((0, 0), 1, &board, &word);
        println!("{shit:?}");

        context.result
    }
}
// @leet end
