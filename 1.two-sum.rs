// @leet start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let num_nums = nums.len();

        'main_loop: for left in 0..num_nums {
            let mut right = left + 1;

            while right < num_nums {
                // println!("working on {left}, {right}");
                if nums[left] + nums[right] == target {
                    result.push(left as i32);
                    result.push(right as i32);
                    return result;
                } else {
                    right += 1;
                }
            }
        }

        return result;
    }
}
// @leet end
