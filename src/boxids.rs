use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn get_boxids_checksum(input: &String) -> i32 {
    let input_lines = input.split("\n");
    let mut duplicates: i32 = 0;
    let mut triplets: i32 = 0;

    for line in input_lines {
        let mut char_count: HashMap<char, i32> = HashMap::new();
        for char in line.chars() {
            let this_char = char_count.entry(char).or_insert(0);
            *this_char += 1;
        }
        let mut found_duplicate: bool = false;
        let mut found_triplet: bool = false;
        for (_, count) in &char_count{
            if found_duplicate && found_triplet {
                break;
            }
            if !found_duplicate && *count == 2 {
                found_duplicate = true;
                duplicates += 1;
            }
            if !found_triplet && *count == 3 {
                found_triplet = true;
                triplets += 1;
            }
        }
    }

    return duplicates * triplets;
}

fn find_common_chars(input: &String) -> String {
    let input_lines: Vec<&str> = input.split("\n").collect();
    let num_ids: usize = input_lines.len();
    let expected_length = input_lines[0].len();

    for i in 0..num_ids{
        let i_chars: Vec<char> = input_lines[i].chars().collect();
        if input_lines[i].len() != expected_length { continue; }

        for j in (i + 1)..num_ids {
            if input_lines[j].len() != expected_length { continue; }
            let j_chars: Vec<char> = input_lines[j].chars().collect();
            let mut differentChars: i32 = 0;

            for ci in 0..i_chars.len() {
                if i_chars[ci] != j_chars[ci] {
                    differentChars += 1;
                }
            }

            if differentChars == 1 {
                let mut answer: String = String::from("");
                for ci in 0..i_chars.len() {
                    if i_chars[ci] == j_chars[ci] {
                        answer.push(i_chars[ci]);
                    }
                }
                return answer;
            }
        }

        
    }
    String::from("Did not find.")
}

pub fn main() {
    let input_filename: String  = String::from("inputs/boxids.txt");
    let mut file = File::open(input_filename).expect("File not found.");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file.");

    let checksum: i32 = get_boxids_checksum(&contents);
    println!("Checksum: {}", checksum);

    let common_chars: String = find_common_chars(&contents);
    println!("id: {}", common_chars);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_boxids_checksum_test() {
        let test_input: &String = 
            &String::from("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab");

        assert_eq!(get_boxids_checksum(test_input), 12);
    }

    #[test]
    fn find_common_chars_test() {
        let test_input: &String = &String::from("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz");

        assert_eq!(find_common_chars(test_input), String::from("fgij"));
    }
}
