use std::fs;

fn main() {
    match sum_values_from_file() {
        Ok(result) => println!("Sum of values: {}", result),
        Err(err) => eprintln!("Error reading file: {:?}", err),
    }
}

fn sum_values_from_file() -> Result<u32, std::io::Error> {
    let file_content = fs::read_to_string("input.txt")?;

    let sum: u32 = file_content
        .lines()
        .filter_map(|line| get_digits(line))
        .sum();

    Ok(sum)
}

fn get_digits(text: &str) -> Option<u32> {
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;

    for c in text.chars() {
        // Check if the character is a digit
        if let Some(digit) = c.to_digit(10) {
            // If first_digit is not set, set it
            if first_digit.is_none() {
                first_digit = Some(digit);
            }
            last_digit = Some(digit);
        }
    }
    // Combine the first and last digits into a single number
    match (first_digit, last_digit) {
        (Some(first), Some(last)) => Some(first * 10 + last),
        _ => None,
    }
}