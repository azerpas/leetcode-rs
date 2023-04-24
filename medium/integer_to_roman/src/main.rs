pub struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut result = String::new();
        let roman = vec![
            (1000, "M"), 
            (900, "CM"), 
            (500, "D"),
            (400, "CD"),
            (100, "C"), 
            (90, "XC"), 
            (50, "L"), 
            (40, "XL"),
            (10, "X"), 
            (9, "IX"), 
            (5, "V"), 
            (4, "IV"), 
            (1, "I")
        ];
        // We decrement num by the value of the largest roman numeral that is less than or equal to num.
        // We then append the corresponding roman numeral to the result string.
        while num > 0 {
            for (value, symbol) in roman.iter() {
                if num >= *value {
                    result.push_str(symbol);
                    num -= value;
                    break;
                }
            }
        }

        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_solution() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}