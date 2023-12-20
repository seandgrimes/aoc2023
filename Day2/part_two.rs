use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

struct Round {
    red: u32,
    green: u32,
    blue: u32
}

struct Game {
    id: u32,
    rounds: Vec<Round>
}

impl Game {
    fn new(id: u32) -> Game {
        Game { id, rounds: Vec::new() }
    }

    // Parses a new instance from a line read from a file
    fn parse(line: &str) -> Game {
        let parts: Vec<&str> = line.split(":").collect();
        let game_id: u32 = parts[0]
            .split(" ")
            .collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();

        let rounds: Vec<&str> = parts[1]
            .trim()
            .split(";")
            .collect();

        let mut game = Game::new(game_id);

        for round in rounds {
            let mut round_entry = Round { red: 0, green: 0, blue: 0 };

            let elements: Vec<&str> = round.trim().split(", ").collect();
            for element in elements {
                let pair: Vec<&str> = element.split(" ").collect();
                let quantity: u32 = pair[0].parse().unwrap();
                let rgb = pair[1];

                match rgb {
                    "red" => round_entry.red = quantity,
                    "green" => round_entry.green = quantity,
                    "blue" => round_entry.blue = quantity,
                    _ => panic!("Unhandled value!")
                }
            }

            game.rounds.push(round_entry);
        }

        return game;
    }

    fn calculate_power(&self) -> u32 {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for round in self.rounds.iter() {
            if round.red > max_red {
                max_red = round.red;
            }
            if round.green > max_green {
                max_green = round.green;
            }
            if round.blue > max_blue {
                max_blue = round.blue;
            }
        }

        max_red * max_green * max_blue
    }
}

fn main() {
    let sum: u32 = read_lines("input.txt")
        .iter()
        .map(|line| Game::parse(line))
        .map(|game| game.calculate_power())
        .sum();

    println!("{}", sum);
}