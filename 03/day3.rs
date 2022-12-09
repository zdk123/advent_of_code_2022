use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::collections::HashMap;



fn char_values(s: &str) -> HashMap<char, u32> {
    let mut char_values = HashMap::new();

    for (i, ch) in s.chars().enumerate() {
        char_values.insert(ch, (i + 1) as u32);
    }

    char_values
}

fn common_chars(left: &str, right: &str) -> String {
    let left_chars: HashSet<char> = left.chars().collect();
    let right_chars: HashSet<char> = right.chars().collect();

    let common_chars: HashSet<char> = left_chars.intersection(&right_chars).copied().collect();

    common_chars.into_iter().collect::<String>()
}



fn main() {
    // Open the file in read-only mode
    let file = File::open("input3.txt").unwrap();
    let reader = BufReader::new(file);

    // Set up alphabet maps
    let alphabet_lower = "abcdefghijklmnopqrstuvwxyz";
    let alphabet_upper = alphabet_lower.to_uppercase();
    let alphabet = format!("{}{}", alphabet_lower, alphabet_upper);

    let char_values = char_values(&alphabet);

    let mut groups = Vec::new();
    let mut group = Vec::new();
    let mut counter = 0;
    let mut total_value = 0;
    // Read the lines of the file
    for line in reader.lines() {
        let line = line.unwrap();
        // Split the line in half
        let (left, right) = line.split_at(line.len() / 2);

        // Find the common prefix of the two halves
        let common: Vec<char> = common_chars(left, right).chars().collect();

        total_value += char_values[&common[0]];
        if counter < 2 {
            group.push(line);
            counter += 1;
        } else {
            group.push(line);
            groups.push(group);
            group = Vec::new();
            counter = 0;
        }
    }
    println!("Total {}", total_value);

    // now iterate through each group and find the 'badge' The letter in common within all three groups //
    let mut total_group_value = 0;
    for group in groups {
        let common: Vec<char> = common_chars(&common_chars(&group[0], &group[1]), &group[2]).chars().collect();
        total_group_value += char_values[&common[0]];
    }
    println!("Total {}", total_group_value);

}