impl Sqrt {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut square : i64 = 0;
        for i in 0..((x / 2) + 2) {
            square = (i as i64) * (i as i64);
            if square == x as i64{
                return i;
            }
            else if square > x as i64{
                return i - 1;
            }
        }
        return 0;
    }

    // Without i64
    pub fn my_sqrt1(x: i32) -> i32 {
        let mut square : i32 = 0;
        for i in 1..((x / 2) + 2) {
            if i > x / i{
                return i - 1;
            }
            else if (i * i) == x {
                return i;
            }
        }
        return 0;
    }

    // Binary search
    pub fn my_sqrt2(x: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = x;
        let mut result: i32 = 0;

        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_squared: i64 = (mid as i64) * (mid as i64);

            if mid_squared == x as i64 {
                return mid;
            }

            if mid_squared < x as i64 {
                left = mid + 1;
                result = mid;
            } else {
                right = mid - 1;
            }
        }
        result
    }
}