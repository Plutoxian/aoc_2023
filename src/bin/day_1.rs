use std::env;
use std::fs;

/* NOTES:
 *  finished part 1 on december 9
 *  first rust code I've ever written, yippee
 *  need to implement part 2 still. I know this is an application
 *  of NFA, but I'm not confident in rust enough to do it. for the future
 */


fn main() {
    let input_file_path: String = match env::args().len() {
        0 | 1 => panic!("No input file path given"),
        2 => env::args().nth(1).unwrap(),
        _ => panic!("Expected at most 1 argument")
    };

    let file: String = fs::read_to_string(input_file_path)
        .expect(&format!("Given file could not be opened or read"));

    let total = part_one(&file);

    println!("Calibration value is {}", total);
}

fn part_one(input: &str) -> u32 {
    let mut total: u32 = 0;
    
    for line in input.lines() {
        for c in line.chars() {
            if c.is_digit(10) {
                total += c.to_digit(10).unwrap() * 10;
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_digit(10) {
                total += c.to_digit(10).unwrap();
                break;
            }
        }
    }
    return total;
}

fn part_two(input: &str) -> u32 {
    // TODO
    return 0;
}