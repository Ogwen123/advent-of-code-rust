use std::fs;

pub fn part1() {
    // correct
    let data: String =
        fs::read_to_string("./src/inputs/2021/day2.txt").expect("reading file failed");

    let split_data = data.split("\n");
    let depths: Vec<&str> = split_data.collect();

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    for command in depths {
        if command.starts_with("forward") {
            //get distance
            let split_command: Vec<&str> = command.split(" ").collect();

            horizontal += split_command[1].trim().to_owned().parse::<i32>().unwrap();
        //unwrap converts Result<i32, error_stuff> to just i32 (like a promise in typescript, with resolve and reject)
        } else if command.starts_with("down") {
            let split_command: Vec<&str> = command.split(" ").collect();

            depth += split_command[1].trim().to_owned().parse::<i32>().unwrap();
        } else if command.starts_with("up") {
            let split_command: Vec<&str> = command.split(" ").collect();

            depth -= split_command[1].trim().to_owned().parse::<i32>().unwrap();
        }
    }
    let result: i32 = horizontal * depth;
    println!("Part 1: {}", result);
}

pub fn part2() {
    let data: String =
        fs::read_to_string("./src/inputs/2021/day2.txt").expect("reading file failed");

    let split_data = data.split("\n");
    let depths: Vec<&str> = split_data.collect();

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for command in depths {
        if command.starts_with("forward") {
            //get distance
            let split_command: Vec<&str> = command.split(" ").collect();
            let value: i32 = split_command[1].trim().to_owned().parse::<i32>().unwrap();
            horizontal += value;
            depth += value * aim;
        //unwrap converts Result<i32, error_stuff> to just i32 (like a promise in typescript, with resolve and reject)
        } else if command.starts_with("down") {
            let split_command: Vec<&str> = command.split(" ").collect();

            aim += split_command[1].trim().to_owned().parse::<i32>().unwrap();
        } else if command.starts_with("up") {
            let split_command: Vec<&str> = command.split(" ").collect();

            aim -= split_command[1].trim().to_owned().parse::<i32>().unwrap();
        }
    }
    let result: i32 = horizontal * depth;
    println!("Part 2: {}", result);
}
