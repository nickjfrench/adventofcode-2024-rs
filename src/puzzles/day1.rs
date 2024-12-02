pub fn solve(input: String) {
    println!("-- Day 1 --");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input.clone()));
}

fn part1(input: String) -> String {
    const DEBUG: bool = false;

    let mut total = 0;
    let (mut col1, mut col2) = split_columns_u32(&input);

    if DEBUG {
        println!("DEBUG: col1 amt: {:?}", col1.len());
        println!("DEBUG: col2 amt: {:?}", col2.len());
    }

    col1.sort();
    col2.sort();

    if col1.len() != col2.len() {
        return "ERROR: Day 1 - Column 1 length does not match Column 2 length"
            .to_string()
    }

    if DEBUG {
        println!("DEBUG: col1 sorted first : {:?}", col1[0]);
        println!("DEBUG: col1 sorted last : {:?}", col1[col1.len() - 1]);
        println!("DEBUG: col2 sorted first : {:?}", col2[0]);
        println!("DEBUG: col2 sorted last : {:?}", col2[col2.len() - 1]);
    }

    for row_number in 0..col1.len() {
        // Calculate Distance from Largest - Smallest.
        if col1[row_number] >= col2[row_number] {
            total += col1[row_number] - col2[row_number];
        } else {
            total += col2[row_number] - col1[row_number];
        }
    }

    total.to_string()
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

fn part2(input: String) -> String {
    "not implemented.".to_string()
}