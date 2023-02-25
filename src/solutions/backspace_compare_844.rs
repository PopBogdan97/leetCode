//! Given two strings s and t, return true if they are equal when both are typed into empty text
//! editors. '#' means a backspace character.
//!
//! Note that after backspacing an empty text, the text will continue empty.
//!
//!  
//!
//!  Example 1:
//!
//!  Input: s = "ab#c", t = "ad#c"
//!  Output: true
//!  Explanation: Both s and t become "ac".
//!
//!  Example 2:
//!
//!  Input: s = "ab##", t = "c#d#"
//!  Output: true
//!  Explanation: Both s and t become "".
//!
//!  Example 3:
//!
//!  Input: s = "a#c", t = "b"
//!  Output: false
//!  Explanation: s becomes "c" while t becomes "b".
//!
//!   
//!
//!   Constraints:
//!
//!       1 <= s.length, t.length <= 200
//!           s and t only contain lowercase letters and '#' characters.
//!
//!            
//!
//!            Follow up: Can you solve it in O(n) time and O(1) space?
//!

mod backspace_compare_844 {
    pub fn backspace_compare(s: String, t: String) -> bool {}
}

#[cfg(test)]
mod tests {
    use super::backspace_compare_844::backspace_compare;

    #[test]
    fn test_example_1() {
        let s = "ab#c".to_string();
        let t = "ad#c".to_string();

        let result = backspace_compare(s, t);

        assert_eq!(result, true);
    }

    #[test]
    fn test_example_2() {
        let s = "ab##".to_string();
        let t = "c#d#".to_string();

        let result = backspace_compare(s, t);

        assert_eq!(result, true);
    }

    #[test]
    fn test_example_3() {
        let s = "a#c".to_string();
        let t = "b".to_string();

        let result = backspace_compare(s, t);

        assert_eq!(result, false);
    }
}
