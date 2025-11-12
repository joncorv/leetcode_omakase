use core::num;
use std::fmt::format;

// @leet start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        // let's check if the number is negative
        let mut start_num: usize = 0;
        if x < 0 {
            start_num = 1;
        }

        let char_array: Vec<char> = format!("{}", x).chars().collect();
        // println!("{:?}", char_array);

        let mut result_string: String = "".to_string();
        let len_array = char_array.len();

        println!(
            "we are starting on: {}, and ending on: {}",
            len_array, start_num
        );

        for i in (start_num..len_array).rev() {
            println!("current iter: {}", i);
            let current_char: char = *char_array.get(i).unwrap();
            println!("operating on this char: {}", current_char);
            result_string.push(current_char);
        }

        println!("result string is: {:?}", result_string);
        let mut result_int_64: i64 = result_string.parse().unwrap();

        if result_int_64 < i32::MIN.into() || result_int_64 > i32::MAX.into() {
            let zero_result: i32 = 0;
            return zero_result;
        } else {
            let mut result_int: i32 = result_string.parse().unwrap();
            if start_num == 1 {
                result_int *= -1;
            }

            let number: i32 = 123;
            return result_int;
        }
    }
}
// @leet end
