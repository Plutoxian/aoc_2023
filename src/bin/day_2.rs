use std::collections::HashMap;
use std::env;
use std::fs;


struct DiceColors {
    red: u32,
    green: u32,
    blue: u32,
}

const DICE_IN_BAG: DiceColors = DiceColors{ red: 12, green: 13, blue: 14 };


fn main() {
    let input_file_path: String = match env::args().len() {
        0 | 1 => panic!("No input file path given"),
        2 => env::args().nth(1).unwrap(),
        _ => panic!("Expected at most 1 argument")
    };

    let file: String = fs::read_to_string(input_file_path)
        .expect(&format!("Given file could not be opened or read"));

    let total = part_one(&file);

    println!("Part one result:");
    println!("total game ID is {}", total);
}

fn part_one(input: &str) -> u32 {
    let mut games: HashMap<u32, Vec<DiceColors>> = HashMap::new();
    for line in input.lines() {
        let mut line_iter = line.split(':');
        let num_game = u32::from_str_radix(
                line_iter.next().unwrap().split(' ').nth(1).unwrap(),
                10).unwrap();
        let mut reveals: Vec<DiceColors> = Vec::new();
        for game in line_iter.next().unwrap().split(';') {
            let mut game_colors = DiceColors{ red: 0,  green: 0, blue: 0 };
            for reveal in game.split(',') {
                let mut reveal_iter = reveal.trim().split(' ');
                let num_color = u32::from_str_radix(
                    reveal_iter.next().unwrap(),
                    10).unwrap();
                let color = reveal_iter.next().unwrap().to_lowercase();
                match color.as_str() {
                    "red" => game_colors.red += num_color,
                    "green" => game_colors.green += num_color,
                    "blue" => game_colors.blue += num_color,
                    _ => ()
                }
            }
            reveals.push(game_colors);
        }
        games.insert(num_game, reveals);
    }

    let mut valid_game_total = 0;
    for (game_id, dice_values) in games {
        let mut valid_game = true;
        for value in dice_values {
            if (value.red > DICE_IN_BAG.red) | (value.green > DICE_IN_BAG.green) | (value.blue > DICE_IN_BAG.blue) {
                valid_game = false;
            }
        }
        if valid_game {
            valid_game_total += game_id;
        }
    }
    
    return valid_game_total;
}
