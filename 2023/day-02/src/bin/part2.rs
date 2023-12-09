use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_int_from_string(string: &str) -> i32 {
    let digits: String = string
        .chars()
        .filter(|char| char.is_ascii_digit())
        .collect();
    digits.parse().unwrap()
}

fn get_min_blocks(game_line: &str) -> i32 {
    let mut game_number_list = game_line.split(':');
    game_number_list.next();
    let games_list = game_number_list.next().unwrap().split(';');

    let mut min_red = 1;
    let mut min_green = 1;
    let mut min_blue = 1;
    for game in games_list.into_iter() {
        let entries = game.split(',');
        for entry in entries.into_iter() {
            let number_of_color = get_int_from_string(entry);

            let over_red = entry.contains("red") && number_of_color > min_red;
            let over_green = entry.contains("green") && number_of_color > min_green;
            let over_blue = entry.contains("blue") && number_of_color > min_blue;

            if over_red {
                min_red = number_of_color;
            }
            if over_blue {
                min_blue = number_of_color;
            }
            if over_green {
                min_green = number_of_color;
            }
        }
    }

    min_red * min_blue * min_green
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;

    let input_list = BufReader::new(file).lines();
    let results: i32 = input_list
        .map(|text_line| get_min_blocks(&text_line.unwrap()))
        .sum();

    println!("resuts: {}", results);
    Ok(())
}
