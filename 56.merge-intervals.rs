// @leet start
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result_overlapping_integers: Vec<Vec<i32>> = Vec::new();
        let mut sorted_intervals = intervals.clone();
        sorted_intervals.sort();
        println!("sorted intervals: {sorted_intervals:?}");

        let (mut left, mut right): (usize, usize) = (0, 1);
        let mut prev_iter_found_overlap = false;

        while right < intervals.len() {
            let left_max = sorted_intervals[right - 1][1];
            let right_min = sorted_intervals[right][0];
            let is_final_iteration = right == intervals.len() - 1;

            let current_iter_found_overlap = left_max >= right_min;
            println!("left_max: {left_max}, right_min: {right_min}, overlap?: {current_iter_found_overlap}");

            // current_iter_found_overlap && prev_iter_found_overlap && is_final_iteration

            if current_iter_found_overlap {
                if is_final_iteration {
                    let good_stuff = vec![sorted_intervals[left][0], sorted_intervals[right][1]];
                    result_overlapping_integers.push(good_stuff);
                    return result_overlapping_integers;
                } else {
                    prev_iter_found_overlap = true;
                    right += 1
                }
            } else {
                if prev_iter_found_overlap {
                    let good_stuff = vec![sorted_intervals[left][0], sorted_intervals[right][1]];
                    result_overlapping_integers.push(good_stuff);
                    left = right;
                    right += 1;
                    prev_iter_found_overlap = false;
                } else {
                    if is_final_iteration {
                    } else {
                        left = right;
                        right += 1;
                        prev_iter_found_overlap = false;
                    }
                }
            }
        }

        result_overlapping_integers
    }
}
// @leet end

