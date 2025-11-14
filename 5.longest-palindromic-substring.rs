// @leet start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut longest_string_size: usize = 0;
        let mut result_string: String = String::new();

        for (index, current_char) in s.chars().enumerate() {
            let mut count: usize = 0;
            loop {
                count += 1;
                let (mut left_index, mut right_index): (isize, isize) =
                    (index - count, index + count);
                let (mut left_char, mut right_char): (char, char);

                if let Some(left) = &s.get(left_char) {
                    left_char = left;
                }

                if let Some(right) = &s.get(index + 1) {
                    right_char = right;
                }

                if left_char == right_char {
                    let current_length_found = (count * 2) + 1;
                    if current_length_found > longest_string_size {
                        longest_string_size = current_length_found;
                        let temp_result_string = &s[left_index..right_index];
                        result_string = temp_result_string.to_string();
                        count += 1
                    }
                } else if count == 1 && (left_char == current_char || right_char == current_char) {
                    if longest_string_size > 1 {
                        break;
                    } else {
                        longest_string_size == 2
                    }
                } else {
                    break;
                }
            }
        }

        return result_string;
    }
}
// @leet end
