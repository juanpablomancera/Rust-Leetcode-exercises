impl LongestCommonPrefix {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut strings : Vec<String> = strs.clone();
        strings.sort();

        let first = strings[0].as_str();
        let last = strings.last().unwrap().as_str();

        let mut idx : usize = 0;
        let mut result = String::new();

        while(idx < first.len()  &&  idx < last.len()){
            if first.as_bytes()[idx] == last.as_bytes()[idx]{
                result.push(first.as_bytes()[idx] as char);
                idx += 1;
            }else {
                break;
            }
        }
        return result;
    }
}