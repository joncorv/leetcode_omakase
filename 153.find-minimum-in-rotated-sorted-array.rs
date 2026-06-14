// @leet start
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut nums_sorted = nums.clone();

        nums_sorted.sort_unstable();
        nums_sorted[0]
    }
}
// @leet end

