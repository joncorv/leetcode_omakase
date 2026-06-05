// @leet start
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let dummy: Vec<Vec<i32>> = Vec::new();
        let index_found: Vec<usize> = Vec::new();

        // fn is_intersecting_interval(interval: &Vec<i32>, new_interval: &Vec<i32>) -> bool {
        //     if
        //
        // }
        //

        // let mut intervals = intervals;

        let mut no_overlap: Vec<Vec<i32>> = Vec::new();
        let mut has_overlap: Vec<Vec<i32>> = Vec::new();

        for interval in intervals {
            println!("interval: {interval:?}");

            let start_inside = new_interval[0] <= interval[1] && new_interval[0] >= interval[0];
            let end_inside = new_interval[1] <= interval[1] && new_interval[1] >= interval[0];

            if start_inside || end_inside {
                has_overlap.push(interval.clone());
            } else {
                no_overlap.push(interval.clone());
            }
        }

        println!("no_overlap: {no_overlap:?}");
        println!("has_overlap: {has_overlap:?}");

        let max_val;
        let min_val;

        for interval in has_overlap {}

        dummy
    }
}
// @leet end

