use std::collections::HashSet;

pub fn task_1() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut total_sum: u32 = 0;
    for line in lines {
        let line_parts = line.split_at(line.len()/2);
        let mut doubled_chars: HashSet<char> = HashSet::new();
        for letter in line_parts.0.chars() {
            if line_parts.1.contains(letter) {
                doubled_chars.insert(letter);
            }
        }

        let char_values: Vec<u32> = doubled_chars.iter().map(|x| get_char_value(x)).collect();
        total_sum += char_values.iter().sum::<u32>();
    }
    println!("Sum of priorities: {}", total_sum);
}

fn get_char_value(c: &char) -> u32 {
    let ascii_value = *c as u32;
    let mut out: u32 = 0;
    if ascii_value < 91 && ascii_value > 64 {
        out = ascii_value - 64 + 26;
    } else if ascii_value > 96 && ascii_value < 123 {
        out = ascii_value - 96;
    }
    out
}
