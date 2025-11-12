// @leet start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result: String = "".to_string();
        let mut min_len: usize = usize::MAX;

        // get min len
        for s in &strs {
            let current_len = s.len();
            if min_len > current_len {
                min_len = current_len;
            }
        }
        println!("The shortest word is {min_len} characters long");

        'all_loops: for (word_index, str) in strs.iter().enumerate() {
            // instatiate letter to match
            let mut current_char = char::default();
            println!("this is the {word_index} iteration");

            for (char_index, character) in str.char_indices() {
                if char_index >= min_len {
                    break 'all_loops;
                }
                // if this is the first word
                if word_index == 0 {
                    current_char = character;
                    println!(
                        "this is the {} index word and it's char we're on is: {}",
                        &word_index, &current_char
                    );
                } else {
                    // if letter matches, do nothing
                    // if doesn't match, break
                    if current_char != character {
                        println!("this char: {character} does not match {current_char}, breaking all loops ");
                        break 'all_loops;
                    }
                }
            }
            // if we're here, the letter matches all 3
            // so append the letter to the current char to the result

            result.push(current_char);
        }

        result
    }
}
// @leet end
