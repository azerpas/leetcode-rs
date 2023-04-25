use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut results: Vec<String> = vec![];
        let letters_per_digits = HashMap::from([
            ('2', vec!["a", "b", "c"]),
            ('3', vec!["d", "e", "f"]),
            ('4', vec!["g", "h", "i"]),
            ('5', vec!["j", "k", "l"]),
            ('6', vec!["m", "n", "o"]),
            ('7', vec!["p", "q", "r", "s"]),
            ('8', vec!["t", "u", "v"]),
            ('9', vec!["w", "x", "y", "z"]),
        ]);

        for digit in digits.chars() {
            let letters = letters_per_digits.get(&digit)
                .expect("Could not find a map for digit");
            if results.is_empty() {
                results = letters.iter().map(|&x| x.to_string()).collect();
            } else {
                let mut temp: Vec<String> = vec![];
                for result in &results {
                    for letter in letters {
                        temp.push(
                            format!("{}{}", result, letter)
                        );
                    }
                }
                results = temp;
            }
        }

        println!("{:?}", results);

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut digits = "23".to_string();
        let mut result = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
        assert_eq!(Solution::letter_combinations(digits), result);
        digits = "".to_string();
        result = vec![];
        assert_eq!(Solution::letter_combinations(digits), result);
        digits = "2".to_string();
        result = vec!["a", "b", "c"];
        assert_eq!(Solution::letter_combinations(digits), result);
    }
}