use std::fs::File;
use std::io::prelude::*;

fn calculate_frequency(input: &String) -> i32 {
    let input_lines = input.split("\n");
    let mut total: i32 = 0;

    for line in input_lines {
        let sign: Option<char> = line.chars().next();
        if sign.is_some() {
            let to_add: i32 = line[1..].parse::<i32>().unwrap();
            if sign.unwrap() == '+' {
                total += to_add;
            }
            else {
                total -= to_add;
            }
        }
    }
    
    total
}

pub fn main() {
    let input_filename: String  = String::from("inputs/frequencyInput.txt");
    let mut file = File::open(input_filename).expect("File not found.");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file.");

    let frequency: i32 = calculate_frequency(&contents);

    println!("{}", frequency);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_frequency_test() {
        let test_input_1: String  = String::from("+5\n-3\n+61\n-289");

        assert_eq!(calculate_frequency(&test_input_1), -226);
    }
}
