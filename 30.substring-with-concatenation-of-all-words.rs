// @leet start
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let len_word = words.clone().first().unwrap().chars().count();
        let len_words = words.len();

        let stopping_point = len_words * len_word - 1;

        for index in 0..stopping_point {
            let mut words_working = words.clone();

            for z in 0..(len_word - 1) {
                let search_point = index;
                let testing_word = &s[(search_point)..(search_point + len_word - 1)];

                if let Some(index) = words_working.iter().position(|x| *x == testing_word) {
                    words_working.remove(index);
                    if words_working.is_empty() {
                        result.push(index as i32);
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        return result;
    }
}
// @leet end
