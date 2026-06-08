use core::num;

// @leet start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        let result: Vec<Vec<i32>> = Vec::new();
        let mut result_hash: HashSet<Vec<i32>> = HashSet::new();
        let num_nums = nums.len();
        let mut nums_sorted: Vec<i32> = nums;
        nums_sorted.sort();
        // println!("nums: {nums:?}");

        // let (mut first, mut second, mut third): (usize, usize, usize) = (0, 1, 2);

        let mut first = 0;
        let mut second = 1;
        let mut third = 2;

        'outer: while first < num_nums - 2 {
            // break if first is too high
            if nums_sorted[first] > 0 {
                break;
            }

            second = first + 1;
            'middle: while second < num_nums - 1 {
                // break if first and second are too high
                // if nums_sorted[first] + nums_sorted[second] > 0 {
                //     first += 1;
                //     continue 'outer;
                // }

                // initialize third while loop
                third = second + 1;

                while third < num_nums {
                    // test val
                    let sum: i32 = nums_sorted[first] + nums_sorted[second] + nums_sorted[third];

                    // success
                    if sum == 0 {
                        // add to result array
                        let mut found =
                            vec![nums_sorted[first], nums_sorted[second], nums_sorted[third]];
                        found.sort();
                        result_hash.insert(found);
                        // first += 1;
                        // continue 'outer;
                        second += 1;
                        continue 'middle;
                    }
                    // break if too big
                    // else if sum > 0 {
                    //     first += 1;
                    //     continue 'outer;
                    // }

                    third += 1
                }
                second += 1;
            }
            first += 1;
        }

        let new_result: Vec<Vec<i32>> = result_hash.into_iter().collect();

        new_result
    }
}
// @leet end
