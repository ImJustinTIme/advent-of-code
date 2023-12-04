use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn filter_numbers(text_line: &str) -> i32 {
    let number_list: Vec<u32> = text_line.chars().filter_map(|a| a.to_digit(10)).collect();
    let part1: String = number_list[0].to_string().to_owned();
    let part2: &str = &number_list[number_list.len() - 1].to_string();
    let num_string = part1 + part2;
    println!("{}", num_string);
    num_string.parse().unwrap()
}

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    let file = File::open("input.txt")?;

    let input_list = BufReader::new(file).lines();
    let results: i32 = input_list
        .map(|text_line| filter_numbers(&text_line.unwrap()))
        .sum();

    println!("results:\n {results}");
    Ok(())
}
