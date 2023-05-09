// TODO need to loop through the drawn numbers and check chronologically not just check is a number is in the drawn numbers list

use std::fs;

fn read_data() -> Vec<String> {
    let data: String =
        fs::read_to_string("./src/inputs/2021/day4.txt").expect("reading file failed");

    let split_data = data.split("\n\r\n"); // use \n\r\n because there are blank lines between each useful part
    let broken_data: Vec<&str> = split_data.collect();
    let data: Vec<String> = broken_data.iter().map(|&s| s.into()).collect();
    return data;
}

fn parse_boards() -> Vec<Vec<Vec<i32>>> {
    let mut boards: Vec<Vec<Vec<i32>>> = Vec::new();

    let mut data = read_data();
    data.remove(0); //remove the drawn numbers
    data.remove(1); //removes the first blank line

    for board in data {
        let mut temp_board: Vec<Vec<i32>> = Vec::new();
        let split_board: Vec<&str> = board.split("\n").collect();
        for line in split_board {
            let mut temp_line: Vec<i32> = Vec::new();
            let split_line: Vec<&str> = line.split(" ").collect();
            for number in split_line {
                if number == "" {
                    continue;
                }
                temp_line.push(number.trim().parse::<i32>().unwrap());
            }
            temp_board.push(temp_line);
        }
        boards.push(temp_board);
    }

    return boards;
}

fn parse_drawn_numbers() -> Vec<i32> {
    let data = read_data();
    let numbers = &data[0]; // & symbol means to borrow the variable
    let split_data: Vec<&str> = numbers.split(",").collect();

    let split_numbers: Vec<i32> = split_data
        .iter()
        .map(|&s| s.trim().parse::<i32>().unwrap())
        .collect();

    return split_numbers;
}

fn check_for_bingo(board: &Vec<Vec<i32>>) -> [String; 2] {
    //let board_length = board.len();
    //check for complete row
    let mut counter = 0;
    for i in board {
        if i[0] == -1 && i[1] == -1 && i[2] == -1 && i[3] == -1 && i[4] == -1 {
            return ["true".to_owned(), "row".to_owned()];
        }
        if board[counter][0] == -1
            && board[counter][0] == -1
            && board[counter][0] == -1
            && board[counter][0] == -1
            && board[counter][0] == -1
        {
            return ["true".to_owned(), "col".to_owned()];
        }
        counter += 1;
    }
    return ["false".to_owned(), "col".to_owned()];
}

pub fn part1() {
    let boards: Vec<Vec<Vec<i32>>> = parse_boards();
    let mut boards_to_change: Vec<Vec<Vec<i32>>> = boards.clone();
    let drawn_numbers: Vec<i32> = parse_drawn_numbers();
    //println!("{:?}", boards);as
    //println!("{:?}", drawn_numbers);
    let mut counter: i32 = 0;
    let mut last_value = 0;
    let mut found_at: String = String::new();
    let number_of_boards: usize = boards_to_change.len(); // can only index vectors using usize type
    for i in 0..boards.len() {
        for j in 0..boards[i].len() {
            for k in 0..boards[i][j].len() {
                if drawn_numbers.contains(&boards[i][j][k]) {
                    boards_to_change[i][j][k] = -1
                }
                last_value = boards[i][j][k];
                if check_for_bingo(&boards_to_change[i])[0] == "true" {
                    found_at = i.to_string() + "-" + j.to_string().as_str();
                }
                counter += 1;
            }
        }
        if found_at != String::new() {
            let indexes: Vec<&str> = found_at.split("-").collect();
            let ref index1 = indexes[0].parse::<usize>().unwrap();
            let ref index2 = indexes[1].parse::<usize>().unwrap();
            let row: &Vec<i32> = &boards[*index1][*index2];
            let mut total: i32 = 0;
            for i in 0..5 {
                total += row[i];
            }
            println!("Part 1: {}", total * last_value)
        }
    }
    println!("done: {}", counter)
}
pub fn part2() {}
