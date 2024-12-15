impl ClimbingStairs {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }

        let mut prev2 = 1;
        let mut prev1 = 2;

        for i in 3..(n + 1){
            let current = prev1 + prev2;
            prev2 = prev1;
            prev1 = current;

        }
        return prev1;
    }
}