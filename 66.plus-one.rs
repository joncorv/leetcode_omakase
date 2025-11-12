// @leet start
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits_clone = digits.clone();
        let num_digits = digits_clone.len() - 1;

        // base case
        if num_digits > 0 {
            for i in ((0..num_digits).rev()) {
                // if i > 0 (aka as long as we're not on the first digit)
                // if digits[i] is between 0-8
                // incriment digits[0]
                // break the for loop because we're DONE;
                // else we assume is a 9
                // digits[i] becomes a 0
                // if i == 0 aka first digit
                // if digits[i] is between 0-8
                // incriment digits[0]
                // else if its a 9
                // make digits[0] = 0
                // insert a 1 at index 0

                // return digits

                match digits[i] {
                    0..8 => {
                        let int_index = i as i32;
                        digits_clone[i] = int_index + 1;
                        // return digits_clone;
                        break;
                    }
                    _ => match i {
                        0 => {
                            digits_clone[0] = 0;
                            // let new_zero_vec: Vec<i32> = vec![0];
                            // return new_zero_vec.append(&mut digits_clone);
                            digits_clone.insert(0, 1);
                            // return digits_clone;
                        }
                        _ => {
                            let int_index = i as i32;
                            digits_clone[i] = int_index + 1;
                            // return digits_clone;
                        }
                    },
                }
            }
        }
        digits_clone
    }
}
// @leet end
