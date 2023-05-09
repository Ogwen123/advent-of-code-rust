use std::fs;
use std::str;

static BINARY_LENGTH: usize = 12;

fn convert_to_base10(binary: String) -> i64 {
    let mut total: i64 = 0;
    let mut number_vec: Vec<i64> = Vec::new();
    let split_binary: Vec<&str> = binary.split("").collect();
    for number in split_binary {
        if number.trim() == "" {
            continue;
        }
        number_vec.push(number.trim().parse::<i64>().unwrap());
    }
    let number_vec_len = number_vec.len();
    let mut counter = 0;
    for number in number_vec {
        total += number * (2 as i64).pow((number_vec_len - 1 - counter) as u32);
        counter += 1;
    }
    return total;
}

pub fn part1() {
    let data: String =
        fs::read_to_string("./src/inputs/2021/day3.txt").expect("reading file failed");

    let split_data = data.split("\n");
    let mut numbers: Vec<&str> = split_data.collect();

    let mut gamma_vec: Vec<&str> = Vec::new();
    let mut epsilon_vec: Vec<&str> = Vec::new();

    for i in 0..BINARY_LENGTH {
        let mut ones: i32 = 0;
        let mut zeros: i32 = 0;
        for number in &mut numbers {
            let current_num_vec: Vec<&str> = number.split("").collect();
            let current_num = current_num_vec[i + 1];
            if current_num == "1".to_string() {
                ones += 1
            } else {
                zeros += 1
            }
        }
        if ones > zeros {
            gamma_vec.push("1");
            epsilon_vec.push("0");
        } else {
            gamma_vec.push("0");
            epsilon_vec.push("1");
        }
    }

    let gamma = convert_to_base10(gamma_vec.join(""));
    let epsilon = convert_to_base10(epsilon_vec.join(""));

    let result: i64 = gamma * epsilon;
    println!("Part 1: {}", result);
}

pub fn part2() {
    let data: String =
        fs::read_to_string("./src/inputs/2021/day3.txt").expect("reading file failed");

    let split_data = data.split("\n");
    let mut numbers: Vec<&str> = split_data.collect();

    let mut match_string: String = String::new();

    let mut o2_string: String = String::new();
    let mut co2_string: String = String::new();

    //O2 filtering
    let mut o2_exclude_list = Vec::new();
    for i in 0..BINARY_LENGTH {
        //check is there is a single matching string

        let mut ones: i32 = 0;
        let mut zeros: i32 = 0;
        for number in &mut numbers {
            if o2_exclude_list.contains(number) {
                continue;
            }
            let current_num_vec: Vec<&str> = number.split("").collect();
            let current_num = current_num_vec[i + 1];
            if current_num == "1".to_string() {
                ones += 1
            } else {
                zeros += 1
            }
        }

        if ones > zeros {
            match_string = match_string + "1";
        } else if ones < zeros {
            match_string = match_string + "0";
        } else {
            match_string = match_string + "1";
        }
        let mut matching_strings: Vec<&str> = Vec::new();
        for number in &mut numbers {
            if number
                .to_string()
                .trim()
                .starts_with(match_string.as_str().trim())
            {
                matching_strings.push(number)
            } else {
                o2_exclude_list.push(number)
            }
        }
        if matching_strings.len() == 1 {
            o2_string = matching_strings[0].to_owned();
            break;
        }
    }

    match_string = String::new();
    //CO2 filtering
    let mut co2_exclude_list = Vec::new();
    for i in 0..BINARY_LENGTH {
        //check is there is a single matching string

        let mut ones: i32 = 0;
        let mut zeros: i32 = 0;
        for number in &mut numbers {
            if co2_exclude_list.contains(number) {
                continue;
            }

            let current_num_vec: Vec<&str> = number.split("").collect();
            let current_num = current_num_vec[i + 1];
            if current_num == "1".to_string() {
                ones += 1
            } else {
                zeros += 1
            }
        }

        if ones > zeros {
            match_string = match_string + "0";
        } else if ones < zeros {
            match_string = match_string + "1";
        } else {
            match_string = match_string + "0";
        }
        let mut matching_strings: Vec<&str> = Vec::new();
        for number in &mut numbers {
            if number.starts_with(match_string.as_str()) {
                matching_strings.push(number);
            } else {
                co2_exclude_list.push(number)
            }
        }
        if matching_strings.len() == 1 {
            co2_string = matching_strings[0].to_owned();
            break;
        }
    }

    let o2 = convert_to_base10(o2_string);
    let co2 = convert_to_base10(co2_string);

    let result: i64 = o2 * co2;
    println!("Part 2: {}", result);
}
