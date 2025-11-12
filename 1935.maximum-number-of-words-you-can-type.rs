// @leet start
impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        // put my code here

        let mut test_chars: Vec<char> = Vec::new();
        let test_words: Vec<&str> = text.split(" ").collect();
        let mut result: i32 = 0;

        for (index, c) in broken_letters.char_indices() {
            test_chars.push(c);
        }

        println!("my test_words are {:?}", &test_words);
        println!("my test_chars are {:?}", &test_chars);

        'test_word_loop: for word in &test_words {
            println!("starting word loop for {}", &word);
            let word_success: bool;

            'test_char_loop: for ch in &test_chars {
                if word.contains(*ch) {
                    continue 'test_word_loop;
                }
            }
            println!("the test loop has successfully completed, no broken_letters in this word");
            result += 1;
        }

        // let myval: i32 = 123;
        // return myval;
        result
    }
}
// @leet end
