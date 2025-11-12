// @leet start
use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // use std::collections::HashMap;
        // start here
        let result: HashMap<i32, i32> = HashMap::new();

        nums.iter().map(|x| {
            result.entry(x).and_modify(|y| y += 1).or_insert(1);
        });

        println!("{result:?}");
        return 123;
    }
}
// @leet end
