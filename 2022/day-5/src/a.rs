pub fn task() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let top_part = parts[0];
    let instructions = parts[1].split("\n").collect::<Vec<&str>>();

    let mut crates: Vec<Vec<char>> = vec![Vec::new(); 9];

    for line in top_part.split("\n") {
        let mut row = 1;
        for line_char in (1..line.len()).step_by(4) {
            let cur_char = line.chars().nth(line_char).unwrap();
            if cur_char != ' ' && cur_char.is_alphabetic() {
               crates[row-1].push(cur_char.clone()); 
            }
            row += 1;
        }
    }

    for instruction in instructions {
        let parts = instruction.split(" ").collect::<Vec<&str>>();
        if parts.len() < 2 { continue }
        let to_move_count = parts[1].parse::<usize>().unwrap();
        let from_col = parts[3].parse::<usize>().unwrap() - 1;
        let to_col = parts[5].parse::<usize>().unwrap() - 1;

        let to_move = crates[from_col].drain(0..to_move_count).collect::<Vec<char>>();
        crates[to_col].splice(0..0, to_move.clone().into_iter().rev().collect::<Vec<char>>());
   }

    println!("{:?}", crates.iter().map(|x| x[0]).collect::<String>());
}
