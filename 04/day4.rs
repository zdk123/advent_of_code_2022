use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Open the file and read
    let file = File::open("input4.txt").unwrap();
    let reader = BufReader::new(file);

    let mut num_contained_intervals = 0;
    let mut num_overlapping_intervals = 0;
    // Read each line of the file
    for line in reader.lines() {
        let line = line.unwrap();

        // Split the line on commas
        let parts: Vec<&str> = line.split(",").collect();
        assert_eq!(parts.len(), 2); // there should only be two columns

        // each number is the end point of a interval
        let interval1: Vec<u32> = parts[0].split("-").map(|s| s.parse().unwrap()).collect();
        let interval2: Vec<u32> = parts[1].split("-").map(|s| s.parse().unwrap()).collect();
        
        // How many intervals are completely contained within its paired interval?
        if (interval1[0] >= interval2[0]) & (interval1[1] <= interval2[1]) { // interval2 contains interval1
            num_contained_intervals += 1;
        } else if (interval1[0] <= interval2[0]) & (interval1[1] >= interval2[1]) { // interval1 contains interval2
            num_contained_intervals += 1;
        }

        // How many intervals overlap at all?
        if (interval1[0] >= interval2[0]) & (interval1[0] <= interval2[1]) { // the start of interval1 falls anywhere in interval2
            num_overlapping_intervals += 1;
        } else if (interval2[0] >= interval1[0]) & (interval2[0] <= interval1[1]) { // the start of interval2 falls anywhere in interval1
            num_overlapping_intervals += 1;
        } else if (interval1[1] >= interval2[0]) & (interval1[1] <= interval2[1] ) { // the end of interval1 falls anywhere in interval2
            num_overlapping_intervals += 1;
        } else if (interval2[1] >= interval1[0]) & (interval2[1] <= interval1[1] ) { // the end of interval2 falls anywhere in interval1
            num_overlapping_intervals += 1;
        }
    }
    println!("Number of pairs that have fully contained intervals: {}", num_contained_intervals);
    println!("Number of pairs that have overlapping intervals: {}", num_overlapping_intervals);

}