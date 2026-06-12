// @leet start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        use std::collections::VecDeque;

        let odd_nodes: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        let even_nodes: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();

        odd_nodes.push_front(root);

        'outer: loop {
            while !even_nodes.is_empty() || !odd_nodes.is_empty() {
                for node in odd_nodes {
                    if let Some(some_node) = node {
                        let test_node = some_node.borrow();
                        println!("test_node: {test_node.val}");
                    }
                }
            }
        }

        if let Some(node) = root {
            // println!("root: {node:?}");

            let current_node = node.borrow();
            println!("RefCell contents: {:?}", current_node.val);
        }

        let dummy: Vec<i32> = Vec::new();
        dummy
    }
}
// @leet end
