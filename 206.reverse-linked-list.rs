// @leet start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head_clone = head.clone();

        let mut dummy = ListNode::new(0);
        println!("{dummy:?}");

        fn dfs(node: &mut Option<Box<ListNode>>) {
            if let Some(some_node) = node {
                println!("current val: {:?}", some_node.val);
                dfs(&mut some_node.next);
            }
            return;
        }

        dfs(&mut head_clone);

        head_clone
    }
}
// @leet end
