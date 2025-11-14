// @leet start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut nums1, mut nums2): (Vec<i32>, Vec<i32>) = (nums1, nums2);
        nums1.extend(nums2.iter());
        nums1.sort();

        let num_nums = nums1.len();

        if num_nums % 2 == 0 {
            let left_index = num_nums / 2;
            let right_index = left_index - 1;

            let left_number = nums1[left_index] as f64;
            let right_number = nums1[right_index] as f64;

            return (left_number + right_number) / 2.0;
        } else {
            let index = num_nums / 2;
            return nums1[index] as f64;
        }
    }
}
// @leet end
