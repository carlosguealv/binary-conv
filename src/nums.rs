// function to convert from binary to decimal
pub fn to_decimal(mut num: i32) -> i32 {
    // variables for the result, remainder, and base
    let mut res: i32 = 0;
    let mut rem: i32;
    let mut base: i32 = 1;

    while num > 0 {
        // store the remainder into a variable
        rem = num % 10;

        // divide number by 10
        num /= 10;

        // multiply the remainder by a base of 2 and add to result
        res += rem * base;

        // multiply the base by 2 to update it to the correct power
        base *= 2;
    }

    // return the result
    return res;
}
