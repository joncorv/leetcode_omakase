// @leet start
impl Solution {
    pub fn is_valid_substring(small: &str, large: &str) -> bool {
        let mut large = large.chars();

        for sm_ch in small.chars() {
            if let Some(found_index) = large.position(sm_ch) {
                large.rem
            }
        }

        false
    }

    pub fn min_window(s: String, t: String) -> String {
        // the rules say this won't happen, but i'm doing it anyway
        if s.is_empty() || t.is_empty() {
            return "".to_string();
        }

        let small;
        let large;

        if s.len() < t.len() {
            small = &s;
            large = &t;
        } else {
            small = &t;
            large = &s;
        }

        let (mut left, mut right): (usize, usize) = (0, small.len());

        while right <= large.len() {
            let mut testing_group: Vec<char> = large.get(left..right).unwrap().chars().collect();

            for ch in small {
                let found_index = testing_group.find(ch);

                if let Some(idx) = testing_group.find(ch) {
                    testing_group.remove(idx);
                } else {
                    break;
                }
            }

            println!("iteration: {:?}", large.get(left..right));
            left += 1;
            right += 1;
        }

        return "dummy".to_string();
    }
}
// @leet end
