pub fn task() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<&str> = input.trim().split("\n").collect();

    let mut count = 0;
    for line in lines {
        let assignments = line.split(",").map(|x| x.split("-").map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
        let range_1 = (assignments[0][0]..=assignments[0][1]).collect::<Vec<i32>>();
        let range_2 = (assignments[1][0]..=assignments[1][1]).collect::<Vec<i32>>();
        for num in range_1 {
            if range_2.contains(&num) {
                count += 1;
                break;
            }
        }
   }
    println!("Count of overlapping at least once: {}", count);
}
