// @leet start
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        // n is inclusive range we traverse
        // k is the number of combinations (len of result[0])

        struct Context {
            result: Vec<Vec<i32>>,
            path: Vec<i32>,
        }

        fn dfs(start: i32, n: i32, k: i32, context: &mut Context) {
            // base case success
            if context.path.len() == k as usize {
                context.result.push(context.path.clone());
                return;
            }

            // recursion that searches smaller range the deeper it searches
            for i in start..=n {
                context.path.push(i.clone());
                dfs(i + 1, n, k, context);
                context.path.pop();
            }
        }

        let mut context = Context {
            result: Vec::new(),
            path: Vec::new(),
        };

        dfs(1, n, k, &mut context);

        context.result
    }
}
// @leet end
