use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

fn main() {
    // Initialize a vector for holding the stacks
    let mut stacks: Vec<VecDeque<char>> = Vec::new();


    let file = File::open("input5.txt").unwrap();
    let reader = BufReader::new(file);

    // Create an iterator over the lines of the file
    let mut lines = reader.lines();

    // Read lines until we hit a line break
    let mut counter = 0;
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if counter == 0 {
            // we only need to initialize stacks once
            // each box is represented by 3 characters plus a space buffer (except for the last box has no space)
            let number_of_stacks = (line.len()+1)/4;
            // Create N stacks and push them onto the vector
            for _ in 0..number_of_stacks {
                let stack = VecDeque::new();
                stacks.push(stack);
            }
        }
        if line == "" {
            // We hit a line break, so do something else
            break;
        }
        // chuck the line by converting to a char vector
        let chars: Vec<char> = line.chars().collect();
        let chunks: Vec<&[char]> = chars.chunks(4).collect();
        for (i, chunk) in chunks.iter().enumerate() {
            if chunk[0] != ' ' {
                // we've got a 'box' so push the character to the bottom/front
                stacks[i].push_front(chunk[1]);
            }
        }
        counter += 1;
    }

    // Pick up with the file again for the instruction set
    for line in lines {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();
        let number_of_moves: i32 = parts[1].parse().unwrap();
        let from: i32 = parts[3].parse().unwrap();
        let to: i32 = parts[5].parse().unwrap();
        let mut boxes_to_move = Vec::new(); // part II, this holds the boxes until we're ready to reverse and remove
        for _ in 0..number_of_moves {
            // TODO if time: check we are not going to pop an empty stack
            let box_to_move = stacks[(from-1) as usize].pop_back().unwrap();
            boxes_to_move.push(box_to_move);
            // stacks[(to-1) as usize].push_back(box_to_move);
        }
        boxes_to_move.reverse();
        for box_to_move in boxes_to_move {
            stacks[(to-1) as usize].push_back(box_to_move);
        }

    }
    // Read off the top of the stacks
    let mut result = String::new();
    for stack in stacks {
        let mut stack = stack;
        result.push(stack.pop_back().unwrap());
    }
    println!("Result: {}", result);
}