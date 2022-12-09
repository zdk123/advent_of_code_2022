use std::fs;
use std::io::{BufReader, BufRead};

fn main() {
    // Open the file in read-only mode
    let file = fs::File::open("input1.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut groups = Vec::new();
    let mut group_sum = 0;

    // Read each line of the file
    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        // Check if the line is empty
        if line.is_empty() {
            // If the line is empty, add the current group's sum to the list of sums
            // and reset the current group's sum to 0
            groups.push(group_sum);
            group_sum = 0;
        } else {
            // If the line is not empty, parse the number on the line and add it to
            // the current group's sum
            let num = line.parse::<i32>().expect("Failed to parse number");
            group_sum += num;
        }
    }

    // If there is a group that hasn't been added to the list of sums, add it now
    if group_sum > 0 {
        groups.push(group_sum);
    }

    // Find the maximum value in the list of sums
    let max_sum = groups.iter().max().expect("Failed to find maximum sum");

    println!("Maximum sum: {}", max_sum);

        // Find the top three values in the list of sums
    //let max_sum = groups.sort(); //.max().expect("Failed to find maximum sum");
    groups.sort_by(|a, b| b.cmp(a));

    // Select the three largest sums
    let mut top_three_sum = 0;
    for i in 0..3 {
        top_three_sum += groups[i];
    }
    println!("Sum of the largest three: {:?}", top_three_sum);
}
