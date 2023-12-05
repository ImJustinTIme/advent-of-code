use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_int_from_string(string: &str) -> i32 {
    let digits: String = string.chars().filter(|char| char.is_digit(10)).collect();
    digits.parse().unwrap()
}

fn check_if_possible(game_line: &str) -> i32 {
    let mut game_number_list = game_line.split(':');
    let game_num = get_int_from_string(game_number_list.next().unwrap());
    let games_list = game_number_list.next().unwrap().split(';');

    for game in games_list.into_iter() {
        let entries = game.split(',');
        for entry in entries.into_iter() {
            let number_of_color = get_int_from_string(&entry);

            let over_red = entry.contains("red") && number_of_color > 12;
            let over_green = entry.contains("green") && number_of_color > 13;
            let over_blue = entry.contains("blue") && number_of_color > 14;

            if (over_red || over_blue || over_green) {
                return 0;
            }
        }
    }

    game_num
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;

    let input_list = BufReader::new(file).lines();
    let results: i32 = input_list
        .map(|text_line| check_if_possible(&text_line.unwrap()))
        .sum();

    println!("resuts: {}", results);
    Ok(())
}
