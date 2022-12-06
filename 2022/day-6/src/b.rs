use std::collections::HashSet;

pub fn task() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();

    let mut char_buff: Vec<char> = Vec::new();
    let mut i = 1;
    for i_char in input.chars() {
        if char_buff.len() > 13 {
            char_buff.remove(0);
        }
        char_buff.push(i_char);
        let char_set: HashSet<&char> = HashSet::from_iter(char_buff.iter().clone());
        if char_set.len() == char_buff.len() && char_set.len() == 14 {
            break;
        }
        i += 1;
    }
    println!("start of message marker found at index: {}", i);
}
