use std::collections::HashMap;

impl RomanToInteger {
    pub fn roman_to_int(s: String) -> i32 {
        let roman_numerals = [
            ("I", 1),
            ("V", 5),
            ("X", 10),
            ("L", 50),
            ("C", 100),
            ("D", 500),
            ("M", 1000),
        ];

        let mut map: HashMap<String, i32> = roman_numerals
            .iter()
            .map(|(k, v)| (k.to_string(), *v))
            .collect();

        let s_array: Vec<char> = s.chars().rev().collect();

        let mut prev : i32 = 0;
        let mut acc : i32 = 0;

        for c in s_array {
            let current_value = map.get(&c.to_string()).unwrap();

            if current_value < &prev {
                acc -= current_value;
            }else {
                acc += current_value;
            }
            prev = *current_value;
        }
        return acc;
    }
}