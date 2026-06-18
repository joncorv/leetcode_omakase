// @leet start
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        struct Context {
            result: Vec<Vec<i32>>,
            path: Vec<i32>,
            total: i32,
            current_num: i32,
        }

        let mut context = Context {
            result: Vec::new(),
            path: Vec::new(),
            total: 0,
            current_num: 0,
        };

        fn dfs(candidates: &[i32], target: &i32, context: &mut Context) {
            //base case failure
            if context.total + context.current_num > *target {
                return;
            }

            // add current_num to path
            if context.current_num != 0 {
                context.path.push(context.current_num.clone());
                context.total += context.current_num.clone();
            }
            // println!("current: {}, path: {:?}", context.current_num, context.path);

            // base case success
            if context.total == *target {
                let mut success_sorted = context.path.clone();
                success_sorted.sort();

                if !context.result.contains(&success_sorted) {
                    context.result.push(success_sorted.clone());
                }
            }

            // recursion to all candidates
            for candidate in candidates {
                context.current_num = candidate.clone();
                dfs(candidates, target, context);
            }

            // post order traversal (none)
            if let Some(last_val) = context.path.pop() {
                // println!("popping!");
                context.total -= last_val;
            }
        }

        dfs(&candidates, &target, &mut context);

        let mut final_result = context.result.clone();
        final_result.sort_unstable();

        final_result
    }
}
// @leet end

