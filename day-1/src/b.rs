use std::fs;

pub fn task_2() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let groups: Vec<&str> = input.split("\n\n").collect();
    
    let mut group_sums: Vec<i32> = Vec::new();
    for group in groups {
        let group_strings = group.split("\n").collect::<Vec<&str>>();
        let group_nums: Vec<i32> = group_strings.iter().map(|x| x.parse::<i32>().unwrap_or(0)).collect();
        let group_sum = group_nums.iter().sum();
        group_sums.push(group_sum); 
    }

    group_sums.sort_by(|a, b| b.cmp(a));
    let top_three_sum: i32 = group_sums[..3].iter().sum();

    println!("Top 3 Elfs with most calories: {}", top_three_sum);
}
