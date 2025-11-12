use core::num;

// @leet start
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let iter_1: Vec<&str> = version1.split(".").collect();
        let iter_2: Vec<&str> = version2.split(".").collect();

        println!("iter_1: {iter_1:#?}");
        println!("iter_2: {iter_2:#?}");

        let iter_1_len = iter_1.len();
        let iter_2_len = iter_2.len();

        let mut num_iterations: usize = 0;
        if iter_1_len > iter_2_len {
            num_iterations = iter_1_len;
        } else {
            num_iterations = iter_2_len;
        }

        println!("num iterations: {num_iterations}");
        let default_val = "0";

        for i in 0..num_iterations {
            let val_1: u32 = iter_1.get(i).unwrap_or(&&default_val).parse().unwrap();
            let val_2: u32 = iter_2.get(i).unwrap_or(&&default_val).parse().unwrap();

            println!("val_1: {val_1}, val_2: {val_2}");

            if val_1 < val_2 {
                return -1;
            } else if val_1 > val_2 {
                return 1;
            }
        }
        return 0;
    }
}
// @leet end
