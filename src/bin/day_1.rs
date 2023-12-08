use std::env;
use std::fs;

fn main() {
    let input_file_path: String = match env::args().len() {
        0 | 1 => panic!("No input file path given"),
        2 => env::args().nth(1).unwrap(),
        _ => panic!("Expected at most 1 argument")
    };

    let file: String = fs::read_to_string(input_file_path)
        .expect(&format!("Given file could not be opened or read"));

    let mut total: u32 = 0;
    
    for line in file.lines() {
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

    println!("Calibration value is {}", total);
}
