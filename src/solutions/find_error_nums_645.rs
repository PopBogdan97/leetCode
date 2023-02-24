//! You have a set of integers s, which originally contains all the numbers from 1 to n.
//! Unfortunately, due to some error, one of the numbers in s got duplicated to another number in
//! the set, which results in repetition of one number and loss of another number.
//!
//! You are given an integer array nums representing the data status of this set after the error.
//!
//! Find the number that occurs twice and the number that is missing and return them in the form of
//! an array.
//!
//!  
//!
//!  Example 1:
//!
//!  Input: nums = [1,2,2,4]
//!  Output: [2,3]
//!
//!  Example 2:
//!
//!  Input: nums = [1,1]
//!  Output: [1,2]
//!
//!   
//!
//!   Constraints:
//!
//!       2 <= nums.length <= 104
//!           1 <= nums[i] <= 104
//!
//!

mod find_error_nums_645 {
    /// The idea is to sort the array and then check that where the number is repeated to finde the
    /// first error and then by checking the absolute value of the two adjacent numbers, you can
    /// find the second error.
    ///
    /// Alternative solution is to use a set to store all the numbers from 1 to n and, then cycle
    /// over the nums array and remove the the numbers from the set. The remaining number will be
    /// the missing one and the one that will be removed twice will be the duplicated one.
    ///
    /// Alternative solution:
    /// 1. Create a vector of boolean of size n and initialize it to false.
    /// 2. Cycle over the nums vector and check if you already accessed the correspondet cell to the
    ///    number in the vector. If you already accessed it you have found the duplicated number.
    /// 3. Iterate over the vector of booleans to find the number that is missing.

    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        // let mut nums = nums;
        // nums.sort();
        // let mut result = vec![0; 2];
        // for i in 1..nums.len() {
        //     if nums[i] == nums[i - 1] {
        //         result[0] = nums[i];
        //     } else if nums[i] - nums[i - 1] > 1 {
        //         result[1] = nums[i] - 1;
        //     }
        // }
        // if result[1] == 0 {
        //     if nums[0] != 1 {
        //         result[1] = 1;
        //     } else {
        //         result[1] = nums.len() as i32;
        //     }
        // }
        // result
        //

        // let nums = nums;
        // let mut result = vec![0; 2];
        // let mut set = (1..=nums.len() as i32).collect::<std::collections::HashSet<i32>>();
        // for i in nums {
        //     let present = set.remove(&i);
        //     if !present {
        //         result[0] = i;
        //     }
        // }
        //
        // result[1] = *set.iter().next().unwrap();
        // result

        let nums = nums;
        let mut result = vec![0; 2];
        let mut v = vec![false; nums.len()];
        for i in nums {
            if !v[(i - 1) as usize] {
                v[(i - 1) as usize] = true;
            } else {
                result[0] = i;
            }
        }

        for b in 0..v.len() {
            if !v[b] {
                result[1] = (b + 1) as i32;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::find_error_nums_645::find_error_nums;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 2, 2, 4];
        let expected = vec![2, 3];
        let result = find_error_nums(nums);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 1];
        let expected = vec![1, 2];
        let result = find_error_nums(nums);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![2, 2];
        let expected = vec![2, 1];
        let result = find_error_nums(nums);
        assert_eq!(result, expected);
    }
}
