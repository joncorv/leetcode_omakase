// @leet start
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let check_val: usize = &nums.len() - (k as usize);

        println!("{check_val}");

        nums[check_val]
    }
}
// @leet end
