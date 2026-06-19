// @leet start
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return Vec::new();
        }
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut visited: Vec<i32> = Vec::new();

        fn dfs(nums: &[i32], result: &mut Vec<Vec<i32>>, visited: &mut Vec<i32>) {
            // base case success!
            // sort visited, check to see if it's already been addded
            let mut sorted_visited = visited.clone();
            sorted_visited.sort();

            if !result.contains(&sorted_visited) {
                result.push(sorted_visited.clone());
            }

            // recursion over valid neighbors
            for num in nums {
                if !visited.contains(num) {
                    visited.push(*num);
                    dfs(nums, result, visited)
                }
            }

            // post order traversal
            visited.pop();
        }

        dfs(&nums, &mut result, &mut visited);

        result
    }
}
// @leet end
