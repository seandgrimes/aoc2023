use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    let mut sum : u32 = 0;

    for line in read_lines("input.txt") {
        let numbers : Vec<_> = line
            .chars()
            .filter(|ch| ch.is_numeric())
            .map(|ch| ch.to_digit(10).unwrap())
            .collect();

        let first : u32 = numbers[0];
        let last : u32 = numbers[numbers.len() - 1];

        sum = sum + 10 * first + last;
    }

    println!("{}", sum)
}