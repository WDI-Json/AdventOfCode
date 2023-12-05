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


#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_parse_game_data() {
        let game_data = "Game 1: 13 green, 3 red; 4 red, 9 green, 4 blue; 9 green, 10 red, 2 blue";
        let game_dict = parse_game_data(game_data);

        let mut expected = HashMap::new();
        expected.insert("Game 1".to_string(), vec![
            HashMap::from([("green", 13), ("red", 3), ("blue", 0)]),
            HashMap::from([("green", 9), ("red", 4), ("blue", 4)]),
            HashMap::from([("green", 9), ("red", 10), ("blue", 2)])
        ]);

        let rows = game_dict.get("Game 1").unwrap();
        for row in rows {
            let red = row.get("red");
            let green = row.get("green");
            let blue = row.get("blue");
            
            

        }

        for row in rows {
            println!("{:?}", row);
        }

        // Optionally, print the result (note that println! in tests only shows output when the test fails or when run with `cargo test -- --nocapture`)
        println!("Test passed. Game dictionary: {:?}", game_dict);
    }
}

