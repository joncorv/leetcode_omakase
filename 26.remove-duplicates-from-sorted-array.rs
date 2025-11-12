use core::num;

// @leet start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0 as i32;
        } else if nums.len() == 1 {
            return 1 as i32;
        }

        let mut num_items: usize = nums.len();
        let mut index: usize = 1;

        while index < num_items {
            if nums[index] == nums[index - 1] {
                nums.remove(index);
                num_items -= 1;
            } else {
                index += 1;
            }
        }

        return nums.len() as i32;
    }
}
// @leet end

