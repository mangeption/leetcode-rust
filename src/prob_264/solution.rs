// Write a program to find the n-th ugly number.

// Ugly numbers are positive numbers whose prime factors only include 2, 3, 5.

// Example:

// Input: n = 10
// Output: 12
// Explanation: 1, 2, 3, 4, 5, 6, 8, 9, 10, 12 is the sequence of the first 10 ugly numbers.

// Note:

//     1 is typically treated as an ugly number.
//     n does not exceed 1690.
pub struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        return Self::get_ugly_at(n);
    }

    fn get_ugly_at(n: i32) -> i32 {
        let mut sorted_ugly: Vec<i32> = vec![1];
        let (mut i2, mut i3, mut i5) = (0usize, 0usize, 0usize);
        let mut count = 0;
        while count < n {
            let (next2, next3, next5) = (
                sorted_ugly[i2] * 2,
                sorted_ugly[i3] * 3,
                sorted_ugly[i5] * 5,
            );
            let mut min = next2;
            if min >= next3 {
                min = next3;
            }
            if min >= next5 {
                min = next5;
            }

            sorted_ugly.push(min);

            if min == next2 {
                i2 += 1;
            }

            if min == next3 {
                i3 += 1;
            }

            if min == next5 {
                i5 += 1;
            }

            count += 1;
        }

        return sorted_ugly[(n - 1) as usize];
    }
}
// 6 10 15
