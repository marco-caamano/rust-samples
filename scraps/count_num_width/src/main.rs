fn main() {
    println!("Hello, world!");
    _ = get_width(15);
}

fn get_width(val: u32) -> u32 {
    let mut width = 0;
    let mut my_val = val;
    loop {
        width += 1;
        my_val /= 10;
        if my_val == 0 {
            break;
        }
    }
    width
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(input = {0, 5, 13, 175, 5_300,45_982 , 987_234, 5_123_142}, expected = {1, 1, 2, 3, 4, 5, 6, 7 })]
    fn test_width(input: u32, expected: u32) {
        let width = get_width(input);
        assert_eq!(width, expected);
    }
}
