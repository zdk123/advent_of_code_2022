use std::fs;
use std::io::{BufReader, BufRead};

fn create_symmetric_matrix(value_diagonal: i32, value_upper_triangular: i32, value_lower_triangular: i32) -> [[i32; 3]; 3] {
    let mut matrix = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            if i == j {
                matrix[i][j] = value_diagonal;
            } else if i < j {
                matrix[i][j] = value_upper_triangular;
            } else {
                matrix[i][j] = value_lower_triangular;
            }
        }
    }
    // finalize the payoff matrix
    matrix[0][2] = value_lower_triangular;
    matrix[2][0] = value_upper_triangular;
    matrix
}

// Define each player's tokens as an enum for easier scoring
fn player1_str_to_index(s: &str) -> usize {
    match s {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        _ => 5
    }
}

fn player2_str_to_index(s: &str) -> usize {
    match s {
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => 5
    }
}

// corrected part 2 feature -> look for the payout associated with the letter
fn player2_str_to_strategy(s: &str) -> i32 {
    match s {
        "X" => 0, //lose
        "Y" => 3, //draw
        "Z" => 6, //win
        _ => 5
    }
}

fn main() {

    // set up scoring matrix
    let score_matrix = create_symmetric_matrix(3, 6, 0);


    println!("{:?}", score_matrix);
    let file = fs::File::open("input2.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    // Read each line of the file
    let mut game_sum = 0;
    let mut game_sum_corrected = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        // collect each move
        let words: Vec<&str> = line.split_whitespace().collect();
        let player1_token = words[0];
        let player2_token = words[1];
        let strategy_token = words[1]; // corrected interpretation of this token

        // convert token to index value
        let player1_ind = player1_str_to_index(player1_token);
        let player2_ind = player2_str_to_index(player2_token);

        // assert that the value is never 5 (unrecognized token)
        assert_ne!(player1_ind, 5);
        assert_ne!(player2_ind, 5);
        
        let desired_outcome = player2_str_to_strategy(strategy_token);
        // Play the move that matches the desired outcome
        let payer2_ind_corrected = score_matrix[player1_ind].iter().position(|&x| x == desired_outcome).unwrap() as usize;
        


        // The round sum is the score (win = 6, lose =0, draw=3)
        game_sum += score_matrix[player1_ind][player2_ind];
        // Plus the shape score (rock(X)=1, paper(Y)=2, sissors(Z)=3)
        game_sum += (player2_ind as i32) + 1;

        // in the corrected game sum
        game_sum_corrected += desired_outcome;
        game_sum_corrected += (payer2_ind_corrected as i32) + 1;
    }

    println!("Total score (original): {:?}", game_sum);
    println!("Total score (corrected): {:?}", game_sum_corrected);

}