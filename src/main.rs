extern crate leetcode;

use leetcode::maximum_subarray::solution;

fn main() {
    let r = solution::Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]);

    println!("{:?}", r);
}
// 7
// 1, 7, 5, 2, 4, 3, 9
// 1 7, 7 5, 5 2, 2 4, 4 3, 3 9
// 1 7 5, 7 5 2, 5 2 4, 2 4 3, 4 3 9
// 1 7 5 2, 7 5 2 4, 5 2 4 3, 2 4 3 9
// 1 7 5 2 4, 7 5 2 4 3, 5 2 4 3 9
// 1 7 5 2 4 3, 7 5 2 4 3 9
// 1 7 5 2 4 3 9


// 4
// 3, 1, 2, 4
// 3 1, 1 2, 2 4
// 3 1 2, 1 2 4,
// 3 1 2 4 


// 1 7 5 2 4 3 9
// 