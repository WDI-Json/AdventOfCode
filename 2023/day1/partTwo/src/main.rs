use std::fs;

fn main() {
    match sum_values_from_file() {
        Ok(result) => println!("Sum of values: {}", result),
        Err(err) => eprintln!("Error reading file: {:?}", err),
    }
}

fn sum_values_from_file() -> Result<u32, std::io::Error> {
    let file_content = fs::read_to_string("../input.txt")?;

    let sum: u32 = file_content
        .split_whitespace()
        .flat_map(|word| extract_and_convert_numbers(word))
        .sum();

    Ok(sum)
}

fn extract_and_convert_numbers(text: &str) -> Option<u32> {
    let mut result = 0;
    let mut current_number = None;

    for c in text.chars() {
        if c.is_digit(10) {
            current_number = Some(current_number.unwrap_or(0) * 10 + c.to_digit(10).unwrap());
        } else if let Some(number) = current_number {
            result += number;
            current_number = None;
        }
    }

    if let Some(number) = current_number {
        result += number;
    }

    Some(result)
}

fn word_to_number(word: &str) -> Option<u32> {
    match word.to_lowercase().as_str() {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "ten" => Some(10),
        _ => None,
    }
}