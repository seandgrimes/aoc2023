use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn is_part_number(x: usize, y: usize, bound_x: usize, bound_y: usize, found: bool, schematic: &Vec<Vec<char>>) -> bool {
    //println!("Checking point ({}, {})", x, y);

    let directions: Vec<(i16, i16)> = vec![
        (-1, 1), (0, 1), (1, 1), 
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), (1, 0)];

    if found {
        return found;
    }

    for point in directions {
        let translated: (i16, i16) = (point.0 + x as i16, point.1 + y as i16);
        if translated.0 < 0 || translated.1 < 0 || translated.0 >= bound_x as i16 || translated.1 >= bound_y as i16 {
            continue;
        }

        let ch = schematic[translated.0 as usize][translated.1 as usize];
        if !ch.is_digit(10) && ch != '.' {
            return true;
        }
    }

    false
}

fn main() {
    let lines = read_lines("input.txt");
    let x = lines[0].len();
    let y = lines.len();

    let mut schematic = vec![vec!['x'; y]; x];
    for (y_coordinate, line) in lines.iter().enumerate() {
        for (x_coordinate, ch) in line.chars().enumerate() {
            schematic[x_coordinate][y_coordinate] = ch;
        }
    }

    let mut part_number: String = String::new();
    let mut is_part = false;
    let mut sum = 0;

    for y_coordinate in 0..y {
        for x_coordinate in 0..x {
            let square = schematic[x_coordinate][y_coordinate];
            if square.is_digit(10) {
                is_part = is_part_number(x_coordinate, y_coordinate, x, y, is_part, &schematic);
                part_number.push(square);
                continue;
            } 

            if part_number.len() > 0 {
                if is_part {
                    sum += part_number.parse::<i32>().unwrap();
                }
                part_number = String::new();
                is_part = false;
            }
        }
    }

    println!("Sum: {}", sum);

}