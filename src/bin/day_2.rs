use std::collections::HashMap;
use std::env;
use std::fs;

/* NOTES:
 *  finished this one on december 10, still pretty far behind.
 *  need to finish reading the rust book, currently on chapter 10.
 *  lots of stuff im still just winging.
 * 
 *  parsing the file was hell, no idea how to do it well in rust.
 *  lots of unwraps in my code still, I think I'm supposed to avoid them.
 * 
 *  if I tried to optimize this further, I'd change the parsing to only get the
 *  highest value of each color. I made it record all games in part 1 as I didn't
 *  know part 2 yet. oh well
 */


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

    println!("Part one result:");
    println!("total game ID is {}", part_one(&file));

    println!("Part two result:");
    println!("total dice powers is {}", part_two(&file));
}

fn part_one(input: &str) -> u32 {
    let games = parse_input_games(input);
    
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

fn part_two(input: &str) -> u32 {
    let games = parse_input_games(input);
    
    let mut min_dice_power_total = 0;
    for (_, dice_values) in games {
        let mut min_dice = DiceColors{ red: 0, green: 0, blue: 0 };
        for value in dice_values {
            if min_dice.red < value.red {
                min_dice.red = value.red;
            }
            if min_dice.green < value.green {
                min_dice.green = value.green;
            }
            if min_dice.blue < value.blue {
                min_dice.blue = value.blue;
            }
        }
        min_dice_power_total += min_dice.red * min_dice.green * min_dice.blue;
    }
    
    return min_dice_power_total;
}

fn parse_input_games(input: &str) -> HashMap<u32, Vec<DiceColors>>{
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
    return games;
}
