// @leet start
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() < 2 {
            return intervals.clone();
        }
        let mut result_overlapping_integers: Vec<Vec<i32>> = Vec::new();
        let mut sorted_intervals = intervals.clone();
        sorted_intervals.sort();
        println!("sorted intervals: {sorted_intervals:?}");

        let (mut left, mut right): (usize, usize) = (0, 1);
        let mut prev_iter_found_overlap_max: Option<i32> = None;

        while right < intervals.len() {
            let left_max = sorted_intervals[right - 1][1];
            let right_min = sorted_intervals[right][0];
            let is_final_iteration = right == intervals.len() - 1;
            let current_iter_found_overlap = left_max >= right_min;

            println!("left_max: {left_max}, right_min: {right_min}, overlap?: {current_iter_found_overlap}");

            // current_iter_found_overlap && prev_iter_found_overlap && is_final_iteration

            if is_final_iteration {
                if current_iter_found_overlap {
                    let max_val = sorted_intervals[left][1].max(sorted_intervals[right][1]);
                    let good_stuff = vec![sorted_intervals[left][0], max_val];
                    result_overlapping_integers.push(good_stuff);

                    break;
                } else if !current_iter_found_overlap && prev_iter_found_overlap_max.is_some() {
                    let good_stuff =
                        vec![sorted_intervals[left][0], sorted_intervals[right - 1][1]];
                    let good_stuff_2 = sorted_intervals[right].clone();
                    result_overlapping_integers.push(good_stuff);
                    result_overlapping_integers.push(good_stuff_2);

                    break;
                } else {
                    let good_stuff_1 = sorted_intervals[left].clone();
                    let good_stuff_2 = sorted_intervals[right].clone();
                    result_overlapping_integers.push(good_stuff_1);
                    result_overlapping_integers.push(good_stuff_2);

                    break;
                }
            }

            if current_iter_found_overlap {
                // overlap found, right += 1

                right += 1;
                prev_iter_found_overlap_max = true;
            } else if prev_iter_found_overlap_max && !current_iter_found_overlap {
                // previious overlap, but right isn't overlapping

                let good_stuff = vec![sorted_intervals[left][0], sorted_intervals[right - 1][1]];
                result_overlapping_integers.push(good_stuff);
                left = right;
                right += 1;
                prev_iter_found_overlap_max = false;
            } else {
                // neither were found - previously or now

                let good_stuff = sorted_intervals[left].clone();
                result_overlapping_integers.push(good_stuff);
                left = right;
                right += 1;
                prev_iter_found_overlap_max = false;
            }
        }

        result_overlapping_integers
    }
}
// @leet end
