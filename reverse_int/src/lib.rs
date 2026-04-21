#![allow(dead_code, unused)]
struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let string = x.to_string();
        let mut is_negative = false;

        let reverse_string = string
            .chars()
            .rev()
            .map(|chars| {
                if chars == '-' {
                    is_negative = true;
                    ' '
                } else {
                    chars
                }
            })
            .collect::<String>();

        if let Ok(result) = reverse_string.trim().parse::<i32>() {
            if is_negative { -result } else { result }
        } else {
            0
        }
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_reverse() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
    }
}
