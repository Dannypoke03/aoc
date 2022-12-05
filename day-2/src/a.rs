use std::{fs, collections::HashMap};


pub fn task_1() {
    let points: HashMap<&str, i32> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let hand_encoding: HashMap<&str, i32> = HashMap::from([("X", 0), ("Y", -1), ("Z", 1), ("A", 0), ("B", -1), ("C", 1)]);
    let rounds_string = fs::read_to_string("src/input.txt").unwrap(); 
    let rounds_temp = rounds_string.split("\n").collect::<Vec<&str>>();
    let mut rounds: Vec<Vec<&str>> = rounds_temp.iter().map(|x| x.split(" ").collect::<Vec<&str>>()).collect();
    rounds = rounds.iter().filter(|x| x.len() == 2).map(|x| x.to_owned()).collect();

    let mut round_scores: Vec<i32> = Vec::new();
    for round in rounds {
        let mut round_score = points.get(round[1]).unwrap().clone();    
        let their_hand = hand_encoding.get(round[0]).unwrap();
        let my_hand = hand_encoding.get(round[1]).unwrap();
        let round_result = calc_winner(*their_hand, *my_hand);
        if round_result == 2 {
            round_score += 3;
        } else if round_result == *my_hand {
            round_score += 6;
        }
        round_scores.push(round_score.clone());
    }

    let final_score: i32 = round_scores.iter().sum();
    println!("Final Score: {}", final_score);
}

fn calc_winner(a: i32, b: i32) -> i32 {
    let mut out = 2;
    if a != b {
        if a.abs() == b.abs() {
            out = std::cmp::max(a,b);
        } else {
            out = std::cmp::min(a,b);
        }
    } 
    return out;
}
