use std::collections::HashMap;
use std::fs; 

fn main() {
    let valid_games = sum_of_valid_games("../input.txt");
    println!("Sum of valid games: {}", valid_games);
}

fn sum_of_valid_games(filename: &str) -> u32 {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let input_strings = contents.lines();

    let mut total_sum = 0;
    for line in input_strings {
        let line_hashmap = parse_game_data(line);
        if let Some(game_number) = return_power(&line_hashmap) {
            total_sum += game_number;
        }
    }
    total_sum
}
fn parse_game_data(game_data: &str) -> HashMap<String, Vec<HashMap<&str, u32>>> {
    let mut result = HashMap::new();
    let parts: Vec<&str> = game_data.split(": ").collect();
    let game_id = parts[0].to_string();

    let rows = parts[1].split(';')
        .map(|row| {
            row.trim().split(',').map(|color_count| {
                let color_count_parts: Vec<&str> = color_count.trim().split(' ').collect();
                (color_count_parts[1], color_count_parts[0].parse::<u32>().unwrap())
            })
            .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>();

    result.insert(game_id, rows);
    result
}

fn return_power(hashmap: &HashMap<String, Vec<HashMap<&str, u32>>>) -> Option<u32> {
    let game_key = hashmap.keys().next().unwrap();

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    if let Some(rows) = hashmap.get(game_key) {
        for row in rows {
            let red = *row.get("red").unwrap_or(&0);
            let green = *row.get("green").unwrap_or(&0);
            let blue = *row.get("blue").unwrap_or(&0);
            max_red = max_red.max(red);
            max_green = max_green.max(green);
            max_blue = max_blue.max(blue);
        }
    }

    if max_red == 0 && max_green == 0 && max_blue == 0 {
        None
    } else {
        Some(max_red * max_green * max_blue)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game_data() {
        let game_data = "Game 1: 13 green, 3 red; 4 red, 9 green, 4 blue; 9 green, 10 red, 2 blue";
        let game_dict = parse_game_data(game_data);
        assert_eq!(return_power(&game_dict), Some(48));

        // Optionally, print the result (note that println! in tests only shows output when the test fails or when run with `cargo test -- --nocapture`)
        println!("Test passed. Game dictionary: {:?}", game_dict);
    }
}

