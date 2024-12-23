pub struct KidsWithCandies;
impl KidsWithCandies {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut result : Vec<bool> = Vec::new();

        let mut max : i32 = 0;
        for num in &candies {
            if num >= &max {
                max = *num;
            }
        }

        for num in candies.iter(){
            if (num + extra_candies) >= max {
                result.push(true);
            }else{
                result.push(false)
            }
        }
        result
    }
}