use std::collections::HashMap;
use std::fs; 

fn main() {
    let valid_games = sum_of_valid_games("../input.txt");
    println!("Sum of valid games: {}", valid_games);
}

fn sum_of_valid_games(filename: &str) -> i32 {
    //Grab file from dirpath
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let input_strings = contents.split("\n");
    
    //Cleanup strings
    for line in input_strings {
        let line_hashmap = parse_game_data(line);
        
    }
    //
    0
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

fn is_round_valid(hashmap: &HashMap<String, Vec<HashMap<&str, u32>>>) -> bool {
    let rows = hashmap.get("Game 1").unwrap();
    println!("isround {:?}", hashmap.keys());
    for row in rows {
        let red = row.get("red");
        let green = row.get("green");
        let blue = row.get("blue");
        //only 12 red cubes, 13 green cubes, and 14 blue cubes
        if red > Some(&12) || green > Some(&13) || blue > Some(&14) {
            return false;
        }
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game_data() {
        let game_data = "Game 1: 13 green, 3 red; 4 red, 9 green, 4 blue; 9 green, 10 red, 2 blue";
        let game_dict = parse_game_data(game_data);
        assert_eq!(is_round_valid(&game_dict), true);

        // Optionally, print the result (note that println! in tests only shows output when the test fails or when run with `cargo test -- --nocapture`)
        println!("Test passed. Game dictionary: {:?}", game_dict);
    }
}

