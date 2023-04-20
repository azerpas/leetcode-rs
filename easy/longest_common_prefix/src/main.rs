pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }
        // We take the first word in its entirety
        let mut prefix = strs[0].clone();
        // We iterate over the rest of the words
        for word in strs.iter().skip(1) {
            // We iterate over the characters of the prefix
            for (i, prefix_char) in prefix.chars().enumerate() {
                // We check if the 'prefix_char' is the same char as
                // the char at position 'i' in the iterated word
                if prefix_char != word.chars().nth(i).unwrap_or(' ') {
                    prefix.truncate(i);
                    break;
                }
            }
        }

        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let mut strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
        assert_eq!(Solution::longest_common_prefix(strs), "fl".to_string());
        strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        assert_eq!(Solution::longest_common_prefix(strs), "".to_string());
    }
}