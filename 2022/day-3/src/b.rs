pub fn task_2() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut grouped_lines: Vec<Vec<&str>> = vec![Vec::new(); lines.len()/3];

    for i in 0..lines.len()-1 {
        grouped_lines[i/3].push(lines[i]);
    }
    
    let mut group_chars: Vec<char> = Vec::new();
    for group in grouped_lines {
        for cur_char in group[0].chars() {
            if group[1].contains(cur_char) && group[2].contains(cur_char) {
                group_chars.push(cur_char.clone());
                break;
            }
        }
    }
    let char_values = group_chars.iter().map(|x| get_char_value(*x));

    println!("Sum of group chars: {}", char_values.sum::<u32>());

}

fn get_char_value(c: char) -> u32 {
    let ascii_value = c as u32;
    let mut out: u32 = 0;
    if ascii_value < 91 && ascii_value > 64 {
        out = ascii_value - 64 + 26;
    } else if ascii_value > 96 && ascii_value < 123 {
        out = ascii_value - 96;
    }
    out
}
