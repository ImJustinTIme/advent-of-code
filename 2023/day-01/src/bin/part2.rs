use std::fs::File;
use std::io::{BufRead, BufReader};

use fancy_regex::Regex;

fn find_numbers(input: &str) -> String {
    let re: Regex =
        Regex::new(r"(?=(?:one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9|10))")
            .unwrap();

    let result = re.captures(input).unwrap();

    for capture in result.iter() {
        println!("{}", capture.get(0));

        // match caps.get(0).map_or("", |m| m.as_str()) {
        //     "one" => "1",
        //     "two" => "2",
        //     "three" => "3",
        //     "four" => "4",
        //     "five" => "5",
        //     "six" => "6",
        //     "seven" => "7",
        //     "eight" => "8",
        //     "nine" => "9",
        //     _ => "",
        // }
        // .to_string()
    }

    result.to_string()
}

fn filter_numbers(text_line: &str) -> i32 {
    println!("{}", text_line);
    let filtered_text_line = find_numbers(text_line);

    let number_list: Vec<u32> = filtered_text_line
        .chars()
        .filter_map(|a| a.to_digit(10))
        .collect();
    let part1: String = number_list[0].to_string().to_owned();
    let part2: &str = &number_list[number_list.len() - 1].to_string();
    let num_string = part1 + part2;
    println!("{}", num_string);
    num_string.parse().unwrap()
}

fn main() -> std::io::Result<()> {
    let file = File::open("sample2.txt")?;

    let input_list = BufReader::new(file).lines();
    let results: i32 = input_list
        .map(|text_line| filter_numbers(&text_line.unwrap()))
        .sum();

    println!("results: {results}");
    Ok(())
}
