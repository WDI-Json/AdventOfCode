use std::fs;

fn main() {
    match sum_values_from_file() {
        Ok(result) => println!("Sum of values: {}", result),
        Err(err) => eprintln!("Error reading file: {:?}", err),
    }
}

fn sum_values_from_file() -> Result<u32, std::io::Error> {
    let file_content = fs::read_to_string("../input.txt")?;

    let sum: u32 = "two1nine" //file_content
        .lines()
        .filter_map(|line| get_digits(line))
        .sum();

    Ok(sum)
}

fn get_digits(text: &str) -> Option<u32> {
    let groups: Vec<&str> = text.split(|c: char| c.is_alphabetic() != text.chars().next().unwrap().is_alphabetic()).collect();

    println!("{:?}", groups);

    if let (Some(first), Some(last)) = (groups.first(), groups.last()) {
        let first_value = if first.chars().next().unwrap().is_alphabetic() {
            word_to_number(first)
        } else {
            first.parse::<u32>().ok()
        };
        
        let last_value = if last.chars().last().unwrap().is_alphabetic() {
            word_to_number(last)
        } else {
            last.parse::<u32>().ok()
        };

        return first_value.and_then(|f| last_value.map(|l| f * 10 + l));
    }

    None
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
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_digits_two1nine() {
        assert_eq!(
            get_digits("two1nine"),
            Some(29)
        );
    }

    #[test]
    fn test_get_digits_7pqrstsixteen() {
        assert_eq!(
            get_digits("7pqrstsixteen"),
            Some(76)
        );
    }
}