pub fn solve(input: String) {
    println!("-- Day 1 --");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input.clone()));
}

fn part1(input: String) -> String {
    let (col1, col2) = get_columns_and_sort(&input);

    calc_total_distance(col1, col2).to_string()
}

fn part2(input: String) -> String {
    let (col1, col2) = get_columns_and_sort(&input);
    
    calc_total_sim_score(col1, col2).to_string()
}

fn get_columns_and_sort(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (mut col1, mut col2) = split_columns_u32(&input);

    col1.sort();
    col2.sort();

    (col1, col2)
}

fn calc_row_distance(col1: u32, col2: u32) -> u32 {
    // Find the largest, then calculate distance.
    if col1 >= col2 {
        col1 - col2
    } else {
        col2 - col1
    }
}

fn calc_total_distance(col1: Vec<u32>, col2: Vec<u32>) -> u32 {
    let mut total = 0;
    for row_number in 0..col1.len() {
        total += calc_row_distance(col1[row_number], col2[row_number]);
    }
    total
}

fn split_columns_u32(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut col1: Vec<u32> = Vec::new();
    let mut col2: Vec<u32> = Vec::new();

    for line in input.lines() {
        let columns: Vec<&str> = line.split("   ").collect();
        col1.push(str_to_u32(columns[0]));
        col2.push(str_to_u32(columns[1]));
    }

    (col1, col2)
}

fn str_to_u32(s: &str) -> u32 {
    s.parse::<u32>().unwrap()
}

fn count_instances(left: u32, col2: &[u32]) -> u32 {
    col2.iter().filter(|x| **x == left).count() as u32
}

fn calc_sim_score(item: u32, count: u32) -> u32 {
    item * count
}

fn calc_total_sim_score(col1: Vec<u32>, col2: Vec<u32>) -> u32 {
    let mut total = 0;
    for row in col1 {
        total += calc_sim_score(row, count_instances(row, &col2))
    }
    total
}