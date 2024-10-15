fn add_thousand_separator(val: f64) -> String {
    let mut result = format!("{:.2}", val);
    let len = result.len();
    if val.floor() < 1000.0 {
        // there is no need for Thousand separator
        return result;
    }

    let dot_pos = match result.find('.') {
        Some(pos) => pos,
        None => len - 1,
    };
    if dot_pos < 3 {
        return result;
    }
    let mut offset = dot_pos - 3;
    loop {
        result.insert(offset, ',');
        if offset > 3 {
            offset -= 3;
        } else {
            break;
        }
    }

    result
}

// there is no center padding, so manually generate it
fn center_string(val: &str, width: usize) -> String {
    let len: usize = val.len();
    if len >= width {
        // there is no padding that can be generated
        // just return the same string...
        return val.to_string();
    }
    let pad_size = (width - len) / 2;
    let pad = " ".repeat(pad_size);
    if len % pad_size == 0 {
        format!("{}{}{}", pad, val, pad)
    } else {
        // adding extra space to round out if it is not an
        // even split
        format!("{}{} {}", pad, val, pad)
    }
}

fn main() {
    println!("Hello, format samples!\n");

    println!("+{}+{}+", "-".repeat(42), "-".repeat(42));
    println!(
        "| {:40} | {:>40} |",
        "Right Space Padded String", "Left Space Padded String"
    );
    println!("+{}+{}+", "-".repeat(42), "-".repeat(42));

    let width = 40;
    println!(
        "| {:width$} | {:>width$} |",
        "Right Padding with variable width", "Left Padding with variable width"
    );
    println!("+{}+{}+", "-".repeat(42), "-".repeat(42));

    let another_r = "right padding with vars";
    let another_l = "left padding with vars";
    println!("| {another_r:width$} | {another_l:>width$} |");
    println!("+{}+{}+", "-".repeat(42), "-".repeat(42));

    println!(
        "| {:_<40} | {:_>40} |",
        "Right _ Padded String", "Left _ Padded String"
    );
    println!("+{}+{}+", "-".repeat(42), "-".repeat(42));

    println!(
        "| {} | {} |",
        center_string("Center padded String", 40),
        center_string("Another1 center pad", 40)
    );
    println!("+{}+{}+", "-".repeat(42), "-".repeat(42));

    println!(
        "| {} | {} |",
        center_string("", 40), // pass empty string
        center_string(
            "Very Long String that will not center in the alloted space",
            40
        )
    );
    println!("+{}+{}+", "-".repeat(42), "-".repeat(42));

    println!("| {:>40} | {:>40} |", "Some Number:", 45);
    println!("+{}+{}+", "-".repeat(42), "-".repeat(42));

    let zero_padded = format!("{:08}", 45);

    println!("| {:>40} | {:>40} |", "Zero Padded Number:", zero_padded);
    println!("+{}+{}+", "-".repeat(42), "-".repeat(42));

    let cost: f64 = 12_245_351.7829103;

    println!("| {:>40} | {:>40} |", "Some float:", cost);
    println!("+{}+{}+", "-".repeat(42), "-".repeat(42));

    println!("| {:>40} | {:>40.2} |", "Trimmed float:", cost);
    println!("+{}+{}+", "-".repeat(42), "-".repeat(42));
    // it seems doing rounding to X decimals and padding does not work...
    // use a pre-format call of the float
    println!(
        "| {:>40} | {:>width$} |",
        "Float again:",
        format!("{:.2}", cost)
    );
    println!("+{}+{}+", "-".repeat(42), "-".repeat(42));

    println!(
        "| {:>40} | {:>width$} |",
        "Adding Thousand Separators:",
        add_thousand_separator(cost)
    );
    println!("+{}+{}+", "-".repeat(42), "-".repeat(42));
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(input = {0.112345, 3.34851, 987.7812, 1000.0, 1000.789382, 24_293.374, 4_182_341_982.981_23}, expected = {"0.11", "3.35", "987.78", "1,000.00", "1,000.79", "24,293.37", "4,182,341,982.98"})]
    fn test_thousand_separator(input: f64, expected: &str) {
        let result = add_thousand_separator(input);
        assert_eq!(result, expected.to_string());
    }
}
