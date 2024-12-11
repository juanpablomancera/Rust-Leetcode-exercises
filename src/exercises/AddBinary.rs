impl AddBinary {
    pub fn add_binary(a: String, b: String) -> String {
        let mut array1 : Vec<char> = a.chars().rev().collect();
        let mut array2 : Vec<char> = b.chars().rev().collect();
        let mut carry = false;

        let max_len = array1.len().max(array2.len());
        array1.resize(max_len, '0');
        array2.resize(max_len, '0');

        for i in 0..array1.len() {
            if array1[i] == '0' && array2[i] == '1' {
                if carry{
                    array1[i] = '0';
                } else {
                    array1[i] = '1';
                }
            } else if array1[i] == '1' && array2[i] == '1' {
                if carry {
                    array1[i] = '1';
                } else {
                    array1[i] = '0';
                    carry = true;
                }
            } else if array1[i] == '0' && array2[i] == '0' && carry {
                array1[i] = '1';
                carry = false;
            } else if array1[i] == '1' && array2[i] == '0' && carry{
                array1[i] = '0';
            }
        }
        if carry {
            array1.push('1');
        }
        array1.iter().rev().collect::<String>()
    }
}