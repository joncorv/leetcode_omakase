// @leet start
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        struct Context {
            result: bool,
            path: Vec<char>,
            num_letters: usize,
        }

        let mut context = Context {
            result: false,
            path: Vec::new(),
            num_letters: word.len(),
        };

        fn dfs(index: usize, board: &Vec<Vec<char>>, word: &String, context: &mut Context) {
            // base case more common
            if context.result == true {
                return;
            }

            // base case found solution yay good times USA
            if context.path.len() == context.num_letters {
                context.result == true;
                return;
            }
        }

        context.result
    }
}
// @leet end
