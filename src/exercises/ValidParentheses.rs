use std::collections::HashMap;

impl ValidParentheses {
    pub fn is_valid(s: String) -> bool {
        let mut pairs = HashMap::new();
        pairs.insert(')', '(');
        pairs.insert(']', '[');
        pairs.insert('}', '{');

        let mut vec : Vec<char> = Vec::new();

        for c in s.chars(){
            let b = pairs.get(&c);

            let last = vec.last();
            match last {
                Some(element) => {
                    match b {
                        Some(other) => {
                            if element == other {
                                vec.pop();
                            }else{
                                vec.push(c);
                            }
                        }
                        None => {vec.push(c);}
                    }
                }
                None => {
                    vec.push(c)
                }
            }
        }
        return vec.is_empty();
    }
}