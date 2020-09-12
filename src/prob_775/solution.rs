// We have some permutation A of [0, 1, ..., N - 1], where N is the length of A.

// The number of (global) inversions is the number of i < j with 0 <= i < j < N and A[i] > A[j].

// The number of local inversions is the number of i with 0 <= i < N and A[i] > A[i+1].

// Return true if and only if the number of global inversions is equal to the number of local inversions.

// Example 1:

// Input: A = [1,0,2]
// Output: true
// Explanation: There is 1 global inversion, and 1 local inversion.

// Example 2:

// Input: A = [1,2,0]
// Output: false
// Explanation: There are 2 global inversions, and 1 local inversion.

// Note:

//     A will be a permutation of [0, 1, ..., A.length - 1].
//     A will have length in range [1, 5000].
//     The time limit for this problem has been reduced.

pub struct Solution;
impl Solution {
    pub fn is_ideal_permutation(a: Vec<i32>) -> bool {
        let mut local_inv_count: i32 = 0;
        let l = a.len();
        for i in 1..l {
            match a[i - 1] > a[i] {
                true => local_inv_count += 1,
                false => continue,
            }
        }

        let (_, global_inv_count) = Self::merge_sort(a);

        return global_inv_count == local_inv_count;
    }

    pub fn merge_sort(a: Vec<i32>) -> (Vec<i32>, i32) {
        if a.len() == 1 || a.len() == 0 {
            return (a, 0);
        }
        let mid = a.len() / 2;

        let (sorted_left, left_count) = Self::merge_sort(a[..mid].to_vec());
        let (sorted_right, right_count) = Self::merge_sort(a[mid..].to_vec());

        let (sorted, merged_count) = Self::do_merge(sorted_left, sorted_right);

        return (sorted, merged_count + left_count + right_count);
    }

    pub fn do_merge(left: Vec<i32>, right: Vec<i32>) -> (Vec<i32>, i32) {
        let mut left_counter = 0;
        let mut right_counter = 0;
        let mut result: Vec<i32> = Vec::with_capacity(left.len() + right.len());
        let mut inv_count: i32 = 0;

        let left_len = left.len();
        let right_len = right.len();

        while left_counter < left_len && right_counter < right_len {
            if left[left_counter] > right[right_counter] {
                result.push(right[right_counter]);
                right_counter += 1;
                inv_count += (left_len - left_counter) as i32;
            } else {
                result.push(left[left_counter]);
                left_counter += 1;
            }
        }

        if left_counter < left_len {
            while left_counter < left_len {
                result.push(left[left_counter]);
                left_counter += 1;
            }
        }

        if right_counter < right_len {
            while right_counter < right_len {
                result.push(right[right_counter]);
                right_counter += 1;
            }
        }

        return (result, inv_count);
    }
}
