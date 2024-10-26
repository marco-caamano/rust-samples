const ERR_EMPTY_STRING: &str = "Empty Hex String";
const ERR_INVALID_FORMAT: &str = "Invalid format (must be #XXXXXX)";
const ERR_PARSE_ERROR: &str = "Failed Parsing Hex Value";

#[derive(Debug, PartialEq)]
struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

fn extract_rgb(value: &str) -> Result<Rgb, &str> {
    if value.is_empty() {
        return Err(ERR_EMPTY_STRING);
    }
    if value.len() != 7 || !value.starts_with('#') {
        return Err(ERR_INVALID_FORMAT);
    }

    let red_str = &value[1..3];
    let red = match u8::from_str_radix(red_str, 16) {
        Ok(number) => number,
        Err(_) => return Err(ERR_PARSE_ERROR),
    };

    let green_str = &value[3..5];
    let green = match u8::from_str_radix(green_str, 16) {
        Ok(number) => number,
        Err(_) => return Err(ERR_PARSE_ERROR),
    };

    let blue_str = &value[5..7];
    let blue = match u8::from_str_radix(blue_str, 16) {
        Ok(number) => number,
        Err(_) => return Err(ERR_PARSE_ERROR),
    };

    Ok(Rgb { red, green, blue })
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let result = extract_rgb("");
        assert_eq!(result, Err(ERR_EMPTY_STRING));
    }

    #[test]
    fn test_short() {
        let result = extract_rgb("#ffff");
        assert_eq!(result, Err(ERR_INVALID_FORMAT));
    }

    #[test]
    fn test_long() {
        let result = extract_rgb("#ffffffff");
        assert_eq!(result, Err(ERR_INVALID_FORMAT));
    }

    #[test]
    fn test_missing_hash() {
        let result = extract_rgb("fffffff");
        assert_eq!(result, Err(ERR_INVALID_FORMAT));
    }

    #[test]
    fn test_bad_fmt() {
        let result = extract_rgb("#4312FG");
        assert_eq!(result, Err(ERR_PARSE_ERROR));
    }

    #[test]
    fn test_black() {
        let color = Rgb {
            red: 0,
            green: 0,
            blue: 0,
        };
        let result = extract_rgb("#000000");
        assert_eq!(result, Ok(color));
    }

    #[test]
    fn test_white() {
        let color = Rgb {
            red: 255,
            green: 255,
            blue: 255,
        };
        let result = extract_rgb("#fffFFf");
        assert_eq!(result, Ok(color));
    }

    #[test]
    fn test_color1() {
        let color = Rgb {
            red: 50,
            green: 74,
            blue: 32,
        };
        let result = extract_rgb("#324a20");
        assert_eq!(result, Ok(color));
    }
}
