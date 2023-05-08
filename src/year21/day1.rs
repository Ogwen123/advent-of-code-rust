use std::fs;

pub fn part1() {
    let data: String =
        fs::read_to_string("./src/inputs/2022/day1.txt").expect("reading file failed");

    let split_data = data.split("\n");

    let mut previous_depth: i32 = 0;
    let mut increase_counter: i32 = 0;
    let mut iter_count: i32 = 0;

    for line in split_data {
        let current_depth: i32 = line.trim().to_owned().parse::<i32>().unwrap();

        //let current_depth = match current_depth_raw {
        //    Ok(i32) => i32,
        //    Err(ParseIntError) => panic!("error when parsing int"),
        //};

        if current_depth > previous_depth && iter_count != 0 {
            increase_counter += 1;
        }
        previous_depth = current_depth;
        iter_count += 1;
    }
    println!("Part 1: {}", increase_counter.to_string())
}

pub fn part2() {
    let data: String =
        fs::read_to_string("./src/inputs/2022/day1.txt").expect("reading file failed");

    let split_data = data.split("\n");

    let mut previous_depth_window: i32 = 0;
    let mut increase_counter: i32 = 0;
    let mut iter_count: i32 = 0;

    let depths: Vec<&str> = split_data.collect();
    let mut int_depths: Vec<i32> = Vec::new();

    //convert numbers to strings
    for i in 0..depths.len() {
        int_depths.push(depths[i].trim().to_owned().parse::<i32>().unwrap());
    }

    for i in 0..depths.len() - 2 {
        let current_depth_window = int_depths[i] + int_depths[i + 1] + int_depths[i + 2];
        if iter_count == 0 {
            previous_depth_window = current_depth_window
        } else {
            if current_depth_window > previous_depth_window {
                increase_counter += 1;
            }
            previous_depth_window = current_depth_window;
        }
        iter_count += 1;
    }

    println!("Part 2: {}", increase_counter.to_string())
}
