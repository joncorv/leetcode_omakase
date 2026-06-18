// @leet start
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            let blank = Vec::new();
            return blank;
        }

        let mut permutations = Vec::new();
        let mut visited: Vec<i32> = Vec::new();

        fn dfs(nums: &Vec<i32>, visited: &mut Vec<i32>, permutations: &mut Vec<Vec<i32>>) {
            // base case
            if nums.len() == visited.len() {
                permutations.push(visited.clone());
            }

            // add neighbors to the stack
            for num in nums {
                if !visited.contains(num) {
                    // println!("current: {num:?}");
                    visited.push(*num);
                    dfs(nums, visited, permutations);
                }
            }

            // backtrack -- remove thineself from the stack
            visited.pop();
        }

        dfs(&nums, &mut visited, &mut permutations);
        // println!("permutations: {permutations:?}");

        permutations
    }
}
// @leet end
