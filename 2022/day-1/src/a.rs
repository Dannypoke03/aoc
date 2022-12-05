use std::fs;

pub fn task_1() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let groups: Vec<&str> = input.split("\n\n").collect();
    
    let mut max = 0;
    for group in groups {
        let group_strings = group.split("\n").collect::<Vec<&str>>();
        let group_nums: Vec<i32> = group_strings.iter().map(|x| x.parse::<i32>().unwrap_or(0)).collect();
        let group_sum = group_nums.iter().sum();
        if group_sum > max {
            max = group_sum;
        }
    }

    println!("Elf with most calories: {}", max);
}
