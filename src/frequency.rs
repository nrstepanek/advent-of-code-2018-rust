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

fn find_duplicate_frequency(input: &String) -> Option<i32> {
    let mut past_frequencies: Vec<i32> = vec![0];
    let mut frequency: i32 = 0;
    let mut max_iters = 1000;

    while max_iters > 0 {
        for line in input.split("\n") {
            let sign: Option<char> = line.chars().next();
            if sign.is_some() {
                let to_add: i32 = line[1..].parse::<i32>().unwrap();
                if sign.unwrap() == '+' {
                    frequency += to_add;
                }
                else {
                    frequency -= to_add;
                }

                if past_frequencies.contains(&frequency) {
                    return Some(frequency);
                }
                past_frequencies.push(frequency);
            }
        }

        max_iters -= 1;
        println!("Max iters now: {}, Size of freqs: {}", &max_iters, past_frequencies.len());
    }

    None
}

pub fn main() {
    let input_filename: String  = String::from("inputs/frequencyInput.txt");
    let mut file = File::open(input_filename).expect("File not found.");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file.");

    let frequency: i32 = calculate_frequency(&contents);
    let duplicate_frequency: Option<i32> = find_duplicate_frequency(&contents);

    println!("Frequency: {}", frequency);
    if duplicate_frequency.is_some() {
        println!("Duplicate: {}", duplicate_frequency.unwrap());
    }
    else {
        println!("Duplicate: Not found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_frequency_test() {
        let test_input_1: String = String::from("+5\n-3\n+61\n-289");
        let test_input_2: String = String::from("+1\n+1\n+1");
        let test_input_3: String = String::from("+1\n+1\n-2");
        let test_input_4: String = String::from("-1\n-2\n-3");

        assert_eq!(calculate_frequency(&test_input_1), -226);
        assert_eq!(calculate_frequency(&test_input_2), 3);
        assert_eq!(calculate_frequency(&test_input_3), 0);
        assert_eq!(calculate_frequency(&test_input_4), -6);
    }
    
    #[test]
    fn find_duplicate_frequency_test() {
        let test_input_1: String = String::from("+1\n-1");
        let test_input_2: String = String::from("+3\n+3\n+4\n-2\n-4");
        let test_input_3: String = String::from("-6\n+3\n+8\n+5\n-6");
        let test_input_4: String = String::from("+7\n+7\n-2\n-7\n-4");

        assert_eq!(find_duplicate_frequency(&test_input_1), Some(0));
        assert_eq!(find_duplicate_frequency(&test_input_2), Some(10));
        assert_eq!(find_duplicate_frequency(&test_input_3), Some(5));
        assert_eq!(find_duplicate_frequency(&test_input_4), Some(14));
    }
}
