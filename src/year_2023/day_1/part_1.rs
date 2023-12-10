use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    match read_to_string(filename) {
        Ok(content) => {
            return content.lines().map(|line| line.to_string()).collect()
        }
        Err(e) => {
            panic!("Error reading file: {}", e);
        }
    }
}

fn calibration(context: &Vec<String>) -> Vec<i32> {
    let mut calibration: Vec<i32> = Vec::new();

    for line in context {
        let mut number: Vec<String> = Vec::new();

        for char in line.chars() {
            if char.is_ascii_digit() {
                number.push(char.to_string());
            }
        }

        let first_number = &number[0];
        let end_number = &number.last().expect("Error getting last number");

        let number_together: i32 = format!("{}{}", first_number, end_number).parse().expect("Error parsing number");
        calibration.push(number_together);
    }

    return calibration
}


pub(crate) fn puzzle_part_1() {
    println!("Year 2023, Day 1, Part 1 puzzle");

    let context = read_lines("src/year_2023/day_1/input.txt");

    let calibration = calibration(&context);
    println!("-> Calibration: {:?}", calibration);

    let total_sum = calibration.iter().sum::<i32>();
    println!("-> Total sum: {}", total_sum);
}