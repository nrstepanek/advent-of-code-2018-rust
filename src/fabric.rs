use std::fs::File;
use std::io::prelude::*;
use std::cmp;

struct Claim {
    id: i32,
    left_offset: i32,
    top_offset: i32,
    width: i32,
    height: i32,
}

fn build_claim(line: String) -> Option<Claim> {
    let parts: Vec<&str> = line.split(" ").collect();
    if parts.len() != 4 { return None; }

    let offsets: Vec<&str> = parts[2].split(",").collect();
    let sizes: Vec<&str> = parts[3].split("x").collect();
    let i: i32 = parts[0][1..].parse::<i32>().unwrap();
    let lo: i32 = offsets[0].parse::<i32>().unwrap();
    let to: i32 = offsets[1][..offsets[1].len() - 1].parse::<i32>().unwrap();
    let w: i32 = sizes[0].parse::<i32>().unwrap();
    let h: i32 = sizes[1].parse::<i32>().unwrap();

    Some(Claim { id: i, left_offset: lo, top_offset: to, width: w, height: h })
}

fn build_claims(input: String) -> Vec<Claim> {
    let input_lines = input.split("\n");
    let mut claims: Vec<Claim> = Vec::new();

    for line in input_lines {
        let new_claim: Option<Claim> = build_claim(String::from(line));
        if new_claim.is_some() {
            claims.push(new_claim.unwrap());
        }
    }

    claims
}

fn get_fabric_size(claims: &Vec<Claim>) -> (i32, i32) {
    let mut max_width: i32 = 0;
    let mut max_height: i32 = 0;

    for claim in claims {
        let this_width: i32 = claim.left_offset + claim.width;
        let this_height: i32 = claim.top_offset + claim.height;
        max_width = cmp::max(max_width, this_width);
        max_height = cmp::max(max_height, this_height);
    }

    (max_width, max_height)
}

fn count_overlapping_squares(claims: &Vec<Claim>) -> i32 {
    let mut count: i32 = 0;
    let mut fabric = [[0i32; 1000]; 1000];

    for claim in claims {
        for x in claim.left_offset..claim.left_offset + claim.width {
            for y in claim.top_offset..claim.top_offset + claim.height {
                fabric[x as usize][y as usize] += 1; 
            }
        }
    }

    for x in 0..1000 {
        for y in 0..1000 {
            if fabric[x as usize][y as usize] > 1 { count += 1; }
        }
    }

    count
}

fn find_isolated_claim(claims: &Vec<Claim>) -> Option<i32> {
    let mut fabric = [[0i32; 1000]; 1000];

    for claim in claims {
        for x in claim.left_offset..claim.left_offset + claim.width {
            for y in claim.top_offset..claim.top_offset + claim.height {
                fabric[x as usize][y as usize] += 1; 
            }
        }
    }

    for claim in claims {
        let mut is_isolated: bool = true;
        'outer: for x in claim.left_offset..claim.left_offset + claim.width {
            for y in claim.top_offset..claim.top_offset + claim.height {
                if fabric[x as usize][y as usize] != 1 {
                    is_isolated = false;
                    break 'outer;
                }
            }
        }
        if is_isolated { return Some(claim.id); }
    }

    None
}

pub fn main() {
    let input_filename: String = String::from("inputs/fabricClaims.txt");
    let mut file = File::open(input_filename).expect("File not found.");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file");

    let claims: Vec<Claim> = build_claims(contents);
    let (width, height): (i32, i32) = get_fabric_size(&claims);
    println!("width: {} height: {}", width, height);

    let count: i32 = count_overlapping_squares(&claims);
    println!("overlapping tiles: {}", count);

    let isolated_id: Option<i32> = find_isolated_claim(&claims);
    println!("isolated id: {}", isolated_id.unwrap());
}
