use std::fs::File;
use std::io::{BufRead, BufReader};


fn eval_vec(vec: &Vec<u32>)-> bool {
    let n = vec.len();
    for i in 0..(n-1) {
        if vec[i] >= vec[n-1]  {
            // if any tree in line is taller, then the last tree is blocked
            return false;
        }

    }
    return true; // else it is visible
}

fn eval_scenic_score(vec: &Vec<u32>) -> i32 {
    let n = vec.len();
    let mut score = 0;
    for i in (0..(n-1)).rev() {
        score += 1;
        if vec[i] >= vec[n-1] {
            break;
        }
    }
    return score;
}

fn main() {
    let file = File::open("input8.txt").unwrap();
    let reader = BufReader::new(file);

    // Read each line of the file
    let mut forest = Vec::new();
    let mut sight_lines = Vec::new();
    let mut scenic_score = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        // turn a line into a vector
        let chars: Vec<char> = line.chars().collect();
        let mut vec = Vec::new();
        let mut svec = Vec::new();
        let mut scvec = Vec::new();
        for ch in chars {
            // Convert the character to a number and add it to the vector
            vec.push(ch.to_digit(10).unwrap());
            svec.push(0);
            scvec.push(0);
        }
        forest.push(vec);
        sight_lines.push(svec);
        scenic_score.push(scvec);
    }

    // examine the forest and add sight line information
    let dim = forest.len();
    // in the sightline matrix, 0 = blocked, 1 = visible
    for i in 0..dim {
        for j in 0..dim {
            if i == 0 || j == 0 || i == (dim-1) || j == (dim-1) {
                sight_lines[i][j] = 1; // the outter edge is always visible
            } else { // we have to consider the forest from all directions
                // use closure to grab the column vector
                let col_vec = forest.iter().map(| v | v[j] ).collect::<Vec<_>>();

                let left_vec = forest[i][0..=j].to_vec();
                let left = eval_vec(&left_vec);
                let scenic_left = eval_scenic_score(&left_vec);

                let top_vec = col_vec[0..=i].to_vec();
                let top = eval_vec(&top_vec);
                let scenic_top = eval_scenic_score(&top_vec);

                let mut right_vec = forest[i][j..=(dim-1)].to_vec();
                right_vec.reverse();
                let right = eval_vec(&right_vec);
                let scenic_right = eval_scenic_score(&right_vec);
                

                let mut bottom_vec = col_vec[i..=(dim-1)].to_vec();
                bottom_vec.reverse();
                let bottom = eval_vec(&bottom_vec);
                let scenic_bottom = eval_scenic_score(&bottom_vec);

                sight_lines[i][j] = (left || top || right || bottom) as u32;
                scenic_score[i][j] = scenic_left * scenic_top * scenic_right * scenic_bottom;
            }
        }
    }

    // Count all visible trees
    let mut tree_count = 0;
    for i in 0..dim {
        for j in 0..dim {
            tree_count += sight_lines[i][j];

        }
    }
    println!("{:?}", tree_count);

    // Get highest scenic score
    let mut max_scenic_score = 0;
    for i in 0..dim {
        for j in 0..dim {
            if scenic_score[i][j] > max_scenic_score {
                max_scenic_score = scenic_score[i][j];
            }

        }
    }
    println!("{:?}", max_scenic_score);


}