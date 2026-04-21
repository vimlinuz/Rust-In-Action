struct Solution;
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim_start();
        if s.is_empty() {
            return 0;
        }
        let mut is_negative = false;

        let mut output_string = String::new();

        let mut i = 0;
        for c in s.chars() {
            if i == 0 && c == '-' {
                is_negative = true;
                i += 1;
                continue;
            }
            if i == 0 && c == '+' {
                i += 1;
                continue;
            }

            if i == 0 && c == '0' {
                i += 1;
                continue;
            }

            if c.is_digit(10) {
                output_string.push(c);
                i += 1;
            } else {
                break;
            }
        }

        if output_string.is_empty() {
            return 0;
        } else {
            match output_string.parse::<i32>() {
                Ok(number) => {
                    if is_negative {
                        -number
                    } else {
                        number
                    }
                }
                Err(_) => {
                    if is_negative {
                        i32::MIN
                    } else {
                        i32::MAX
                    }
                }
            }
        }
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
    }
    #[test]
    fn test_4() {
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    }
    #[test]
    fn test_5() {
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), i32::MIN);
    }
}
