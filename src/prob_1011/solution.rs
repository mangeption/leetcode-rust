// A conveyor belt has packages that must be shipped from one port to another within D days.

// The i-th package on the conveyor belt has a weight of weights[i].  Each day, we load the ship with packages on the conveyor belt (in the order given by weights). We may not load more weight than the maximum weight capacity of the ship.

// Return the least weight capacity of the ship that will result in all the packages on the conveyor belt being shipped within D days.

// Example 1:

// Input: weights = [1,2,3,4,5,6,7,8,9,10], D = 5
// Output: 15
// Explanation:
// A ship capacity of 15 is the minimum to ship all the packages in 5 days like this:
// 1st day: 1, 2, 3, 4, 5
// 2nd day: 6, 7
// 3rd day: 8
// 4th day: 9
// 5th day: 10

// Note that the cargo must be shipped in the order given, so using a ship of capacity 14 and splitting the packages into parts like (2, 3, 4, 5), (1, 6, 7), (8), (9), (10) is not allowed.

// Example 2:

// Input: weights = [3,2,2,4,1,4], D = 3
// Output: 6
// Explanation:
// A ship capacity of 6 is the minimum to ship all the packages in 3 days like this:
// 1st day: 3, 2
// 2nd day: 2, 4
// 3rd day: 1, 4

// Example 3:

// Input: weights = [1,2,3,1,1], D = 4
// Output: 3
// Explanation:
// 1st day: 1
// 2nd day: 2
// 3rd day: 3
// 4th day: 1, 1

// Constraints:

//     1 <= D <= weights.length <= 50000
//     1 <= weights[i] <= 500

pub struct Solution;
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, d: i32) -> i32 {
        let mut low: i32 = -2147483648;
        let mut high: i32 = 0;
        // search space [low, high]
        // to search faster -> binary search in [low, high] to find the optimal low
        for w in &weights {
            high += *w;
            low = match low <= *w {
                true => *w,
                false => low,
            };
        }

        while low <= high {
            let mid = (high - low) / 2 + low;
            let mut per_day = 0;
            let mut day_req = 1;

            for w in &weights {
                per_day += *w;
                if per_day > mid {
                    day_req += 1;
                    per_day = *w;
                }
            }

            if day_req > d {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        low
    }
}
