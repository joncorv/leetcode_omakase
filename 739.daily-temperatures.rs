// @leet start
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = temperatures.clone();
        let num_temps = temperatures.len();

        let mut previous_temp = -1000000000;

        'main: for (start, temp) in temperatures.iter().enumerate() {
            let current_temp = temp.clone();

            if current_temp == previous_temp {
                let prev_result = result[start - 1];

                if prev_result == 0 {
                    result[start] = 0;
                } else {
                    result[start] = result[start - 1] - 1;
                }
                continue;
            }
            previous_temp = current_temp.clone();

            for end in ((start + 1)..num_temps) {
                if temperatures[end] > current_temp {
                    result[start] = (end - start) as i32;
                    continue 'main;
                }
            }
            result[start] = 0;
        }

        result
    }
}
// @leet end
