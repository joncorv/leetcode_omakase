// @leet start
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        // start here

        let (dividend, divisor): (f64, f64) = (dividend as f64, divisor as f64);

        let result = dividend / divisor;
        result.floor();
        return result as i32;
    }
}
// @leet end
