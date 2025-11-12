// @leet start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut num_nums: usize = nums.len();
        let mut index: usize = 0;

        while index < num_nums {
            if nums[index] == val {
                nums.remove(index);
                num_nums -= 1;
            } else {
                index += 1;
            }
        }

        return index as i32;
    }
}
// @leet end

