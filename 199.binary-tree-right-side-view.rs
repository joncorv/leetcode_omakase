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
        let mut result: Vec<i32> = Vec::new();

        let mut even_nodes: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        let mut odd_nodes: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();

        even_nodes.push_front(root);

        while !&even_nodes.is_empty() || !&odd_nodes.is_empty() {
            // even nodes processing
            let mut found_even = false;
            while let Some(front_node) = even_nodes.pop_front() {
                if let Some(node) = front_node {
                    let borrow = node.borrow();
                    // let foo = node.as_ptr();
                    // let foo = node.borrow();

                    // println!("foo: {foo:?}");

                    // let borrow_val = borrow.val;
                    // let borrow_right = &borrow.right;
                    // let borrow_left = borrow.right;

                    if !found_even {
                        result.push(borrow.val);
                        found_even = true;
                    }

                    odd_nodes.push_back(borrow.right.clone());
                    // odd_nodes.push_back(borrow_left);
                }
            }

            // odd nodes processing
            // let mut found_odd = false;
            // while let Some(front_node) = odd_nodes.pop_front() {
            //     if let Some(node) = front_node {
            //         let borrow = node.borrow();
            //
            //         if !found_odd {
            //             result.push(borrow.val);
            //             found_odd = true;
            //         }
            //         even_nodes.push_back(&borrow.right);
            //         even_nodes.push_back(&borrow.left);
            //     }
            // }
        }

        // even_nodes.iter().for_each(|x| {
        //
        // }));

        // for node in even_nodes {
        //     if let Some(some_node) = node {
        //         // some_node.borrow();
        //         println!("test_node: {:?}", some_node.borrow().val);
        //     }
        // }

        // if let Some(node) = root {
        //     // println!("root: {node:?}");
        //     let current_node = node.borrow();
        //     println!("RefCell contents: {:?}", current_node.val);
        // }

        result
    }
}
// @leet end
