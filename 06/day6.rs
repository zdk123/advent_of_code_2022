use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {

    // Set up a file
    let file = File::open("input6.txt").unwrap();
    let reader = BufReader::new(file);

    // Why do I have to call unwrap twice to get the line?
    let line = reader.lines().next().unwrap().unwrap();
    // Convert the string to a character vector
    let marker_vector: Vec<char> = line.chars().collect();
    //let packet_size = 3; //part 1
    let packet_size = 13;  //part 2
    for i in packet_size..(marker_vector.len()) {
        let mut packet: HashSet<char> = HashSet::new();
        // A packet is the current and previous `n_packet` characters
        for j in (i-packet_size)..(i+1) {
            packet.insert(marker_vector[j]);
        }
        // Let the HashSet dereplicate the packet for us...
        // Return the first len 4 packer
        if packet.len() == (packet_size+1) {
            println!("Position of first unique char: {}", i+1);
            break
        }
    }
}