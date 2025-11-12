// @leet start
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1: i128 = num1.parse().unwrap();
        let num2: i128 = num2.parse().unwrap();

        let result = num1 * num2;

        println!("{} * {} = {}", num1, num2, result);

        return format!("{}", result).to_string();
    }
}
// @leet end
