//! Given a string s, return the number of homogeneous substrings of s. Since the answer may be too
//! large, return it modulo 109 + 7.
//!
//! A string is homogeneous if all the characters of the string are the same.
//!
//! A substring is a contiguous sequence of characters within a string.
//!
//!
//!
//! Example 1:
//!
//! Input: s = "abbcccaa"
//! Output: 13
//! Explanation: The homogeneous substrings are listed as below:
//! "a"   appears 3 times.
//! "aa"  appears 1 time.
//! "b"   appears 2 times.
//! "bb"  appears 1 time.
//! "c"   appears 3 times.
//! "cc"  appears 2 times.
//! "ccc" appears 1 time.
//! 3 + 1 + 2 + 1 + 3 + 2 + 1 = 13.
//!
//! Example 2:
//!
//! Input: s = "xy"
//! Output: 2
//! Explanation: The homogeneous substrings are "x" and "y".
//!
//! Example 3:
//!
//! Input: s = "zzzzz"
//! Output: 15
//!
//!
//!
//! Constraints:
//!
//!     1 <= s.length <= 105
//!         s consists of lowercase letters.
//!

mod count_homogeneous_1759 {
    /// The function can complete the task by cycling once over the string. The algorithm will have
    /// complexity O(n)
    /// Steps: 
    /// 1. Start from the beginning of the string and count the consecutive homogeneous characters.
    /// 2. As soon as you encounter two different consecutive characters, stop and save the lenght
    ///    of the consecutive homogeneous characters.
    /// 3. Compute all the possible combinations of homogeneous substrings in a string of the saved
    ///    length and homogeneous characters. The formula is: ```sum_(k=1)^n k```.
    ///    Once simplified with Wolframalpha.com, the alternative form is: ```1/2 n (n + 1)```.
    ///    The semplification was made to have less computations during the calculation.
    /// 4. Start counting again and reapeat steps 2&3 until the end of the string.

    pub fn count_homogeneous(s: String) -> i32 {

    }
}

#[cfg(test)]
mod tests {
    use super::count_homogeneous_1759::count_homogeneous;

    
    #[test]
    fn test_example_1() {
        let s = "xy".to_string();

        assert_eq!(2, count_homogeneous(s));
    }

    #[test]
    fn test_example_2() {
        let s = "zzzzz".to_string();

        assert_eq!(15, count_homogeneous(s))
    }

}
