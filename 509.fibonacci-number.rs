// @leet start
impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        }

        let mut prev_1 = 0;
        let mut prev_2 = 1;
        let mut depth = 2;
        let mut sum = 1;

        fn recursion(
            n: i32,
            prev_1: &mut i32,
            prev_2: &mut i32,
            sum: &mut i32,
            depth: &mut i32,
        ) -> i32 {
            if *depth == n + 1 {
                return *sum;
            }

            *sum += *sum;
            *depth += 1;

            recursion(n, sum, depth);

            return *sum;
        }

        recursion(n, &mut sum, &mut depth)
    }
}
// @leet end
