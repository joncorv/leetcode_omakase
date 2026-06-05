// @leet start
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() == 0 || t.len() == 0 || s.len() != t.len() {
            return false;
            println!("edge case killed")
        }

        let mut start: usize = 0;
        let mut end: usize = s.len() - 1;

        let mut s: Vec<char> = s.chars().collect();
        let mut t: Vec<char> = t.chars().collect();

        s.sort();
        t.sort();

        println!("s: {s:?}");
        println!("t: {t:?}");

        s == t
    }
}
// @leet end
