// @leet start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // handle edge cases first
        let total_num_chars = s.len();
        if total_num_chars == 0 {
            return 0;
        } else if total_num_chars == 1 {
            return 1;
        }

        // final number to return
        let result_count: i32 = i32::default();

        // Instantiate vector we will operate on
        let working_vector: Vec<char> = Vec::new();

        // Set count to keep track of higheset number;
        let count: u32 = 0;

        // convert s into an iterable we can access index from
        let s: Vec<char> = s.chars().collect();

        let mut start: usize = 0;
        let mut end: usize = 1;

        loop {
            if start >= end {
                break;
            }

            let end_value: char = *s.get(end).unwrap();

            if working_vector.contains(&end_value) {
                if end < total_num_chars {
                    start += 1;
                    end += 1;
                } else {
                    start += 1;
                }
            }
        }

        return result_count;
    }
}
// @leet end
