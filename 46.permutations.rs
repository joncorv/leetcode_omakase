// @leet start
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut permutations: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();

        fn dfs(nums: &[i32], permutations: &mut Vec<Vec<i32>>, path: &mut Vec<i32>) {
            if nums.len() == path.len() {
                permutations.push(path.clone());
                return;
            }

            for num in nums {
                if !path.contains(num) {
                    path.push(*num);
                    dfs(nums, permutations, path);
                    path.pop();
                }
            }
        }

        dfs(&nums, &mut permutations, &mut path);

        permutations
    }
}
// @leet end

