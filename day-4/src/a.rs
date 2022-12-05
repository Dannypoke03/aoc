pub fn task() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<&str> = input.trim().split("\n").collect();

    let mut count = 0;
    for line in lines {
        let assignments = line.split(",").map(|x| x.split("-").map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
        if (assignments[0][0] <= assignments[1][0] && assignments[0][1] >= assignments[1][1]) || (assignments[0][0] >= assignments[1][0] && assignments[0][1] <= assignments[1][1]) {
            count += 1;
        } 
    }
    println!("Count of fully overlapping: {}", count);
}
