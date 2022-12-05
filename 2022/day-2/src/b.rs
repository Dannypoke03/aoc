use std::{fs, collections::HashMap};

pub fn task_2() {
    let points: HashMap<&str, i32> = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
    let match_points: HashMap<&str, i32> = HashMap::from([("X", 0), ("Y", 3), ("Z", 6)]);
    let rounds_string = fs::read_to_string("src/input.txt").unwrap(); 
    let rounds_temp = rounds_string.split("\n").collect::<Vec<&str>>();
    let mut rounds: Vec<Vec<&str>> = rounds_temp.iter().map(|x| x.split(" ").collect::<Vec<&str>>()).collect();
    rounds = rounds.iter().filter(|x| x.len() == 2).map(|x| x.to_owned()).collect();

    let mut round_scores: Vec<i32> = Vec::new();
    for round in rounds {
        let mut round_score = match_points.get(round[1]).unwrap().clone();    

        if round[1] == "Y" {
            round_score += points.get(round[0]).unwrap();
        } else if round[1] == "X" {
            round_score += points.get(find_hand(round[0], false)).unwrap();
        } else if round[1] == "Z" {
            round_score += points.get(find_hand(round[0], true)).unwrap();
        }
        
        round_scores.push(round_score.clone());
    }

    let final_score: i32 = round_scores.iter().sum();
    println!("Final Score 2: {}", final_score);
}

fn find_hand(given_hand: &str, win: bool) -> &str {
    let hand_orders = vec!["B", "A", "C"];
    let cur_index = hand_orders.iter().position(|&x| x == given_hand).unwrap() as i32;
    let mut target_index: i32;
    if win {
        target_index = cur_index - 1;
    } else {
        target_index = cur_index + 1;
    }

    if target_index > 2 {
        target_index = 0;
    } else if target_index < 0 {
        target_index = 2;
    }
    return hand_orders[target_index as usize];
}
