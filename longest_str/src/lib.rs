pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = Vec::new();
        let mut longest = 0;
        let mut current_length = 0;

        s.chars().for_each(|character| {
            if set.contains(&character) {
                if current_length > longest {
                    longest = current_length;
                }
                let mut index_of_occurance = 0;
                for item in set.clone() {
                    index_of_occurance += 1;
                    if item == character {
                        break;
                    }
                }
                set = set[index_of_occurance..].to_vec();
                current_length = set.len() + 1;
                set.push(character);
            } else {
                current_length += 1;
                set.push(character);
            }
        });
        if current_length > longest {
            longest = current_length;
        }
        longest as i32
    }
}
