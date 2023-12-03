use std::fs;

fn main() {
    let coords = sum_values_from_file("../input.txt");
    println!("Calibration: {}", coords);
}

fn sum_values_from_file(filename: &str) -> i32 {
    //Grab file from dirpath
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let input_strings = contents.split("\n");
    
    let mut sum: i32 = 0;
    for line_text in input_strings {
        sum += get_digits(line_text);
    }

    sum
}


fn get_digits(text: &str) -> i32  {
    let first_num = first_or_last_number(text, false);
    let last_num = first_or_last_number(text, true);

    // Combine the first and last digits into a single number
    let as_num = first_num * 10 + last_num;

    as_num
}

fn first_or_last_number(input: &str, reverse: bool) -> i32 {
    let mut i = 0;

    if reverse {
        for character in input.chars().rev() {
            // Check if the character is a digit
            if character.is_digit(10) {
                return character.to_digit(10).unwrap() as i32;
            }

            // If no digit is found, find word but in reverse
            match match_word_with_value_by_length(input, input.len() - i - 1) {
                // If a word is found, return its value
                Some(digit) => return digit,
                // If no word is found, increment the index
                None => i += 1,
            }
        }
    } else {
        // No reverse then we go left-to-right
        for character in input.chars() {
            if character.is_digit(10) {
                return character.to_digit(10).unwrap() as i32;
            }

            // If no digit is found, check for a plain text number
            match match_word_with_value_by_length(input, i) {
                Some(digit) => return digit,
                None => i += 1,
            }
        }
    }
    0
}

fn word_to_number(word: &str) -> Option<i32> {
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

fn match_word_with_value_by_length(input: &str, i: usize) -> Option<i32> {
    // get the length of each word
    for len in 1..=9 {
        // try to get a substring
        let substring = match input.get(i..i + len) {
            Some(substring) => substring,
            None => continue, 
        };

        if let Some(number) = word_to_number(substring) {
            return Some(number);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_num() {
        assert_eq!(first_or_last_number(&"ab12cd34ef", false), 1);
    }

    #[test]
    fn test_last_num() {
        assert_eq!(first_or_last_number(&"ab12cd34ef", true), 4);
    }

    #[test]
    fn test_match_word_with_value_by_length_one() {
        assert_eq!(match_word_with_value_by_length("one", 0), Some(1));
    }

    #[test]
    fn test_match_word_with_value_by_length_no_match() {
        assert_eq!(match_word_with_value_by_length("abcdefg", 0), None);
    }

    #[test]
    fn test_match_word_with_value_by_length_out_of_bounds() {
        assert_eq!(match_word_with_value_by_length("one", 5), None);
    }

    #[test]
    fn test_match_word_with_value_by_length_longest_match() {
        assert_eq!(match_word_with_value_by_length("onetwothree", 0), Some(1));
    }

    #[test]
    fn test_get_digits_two1nine() {
        assert_eq!(get_digits("two1nine"), 29);
    }
    #[test]
    fn test_get_digits_eightwothree() {
        assert_eq!(get_digits("eightwothree"), 83);
    }
    #[test]
    fn test_get_digits_abcone2threexyz() {
        assert_eq!(get_digits("abcone2threexyz"), 13);
    }
    #[test]
    fn test_get_digits_xtwone3four() {
        assert_eq!(get_digits("xtwone3four"), 24);
    }
    #[test]
    fn test_get_digits_4nineeightseven2() {
        assert_eq!(get_digits("4nineeightseven2"), 42);
    }
    #[test]
    fn test_get_digits_zoneight234() {
        assert_eq!(get_digits("zoneight234"), 14);
    }
    #[test]
    fn test_get_digits_7pqrstsixteen() {
        assert_eq!(get_digits("7pqrstsixteen"), 76);
    }
}

