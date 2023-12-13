use std::fs::read_to_string;
use std::convert::TryFrom;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    let numbers : &Vec<&str> = &vec![
        "one", 
        "two", 
        "three",
        "four", 
        "five", 
        "six", 
        "seven", 
        "eight", 
        "nine"];

    let mut sum : u32 = 0;

    for line in read_lines("input2.txt") {
        let mut entries : Vec<u32> = Vec::new();

        for (ch_idx, ch) in line.chars().enumerate() {
            if ch.is_numeric() {
                let numeric_value = ch.to_digit(10).unwrap();
                entries.push(numeric_value);
                continue;
            }

            for (num_idx, num) in numbers.iter().enumerate() {
                if line[ch_idx..].starts_with(num) {
                    let value: u32 = u32::try_from(num_idx).unwrap() + 1;
                    entries.push(value);
                }
            }
        }

        sum = sum + 10 * entries[0] + entries[entries.len()-1];
    }

    println!("{}", sum);
}