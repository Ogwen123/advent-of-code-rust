use std::io;

mod year21;

fn main() {
    //choose year
    let mut puzzle_date: String = String::new();
    println!("Choose puzzle(YY-D): ");
    io::stdin().read_line(&mut puzzle_date).expect("bad input");
    puzzle_date = puzzle_date.trim().to_owned();

    //split the string
    let puzzle_split: std::str::Split<&str> = puzzle_date.split("-");
    let mut puzzle_year: &str = "";
    let mut puzzle_day: &str = "";
    let mut counter: i32 = 0;
    for parts in puzzle_split {
        if counter == 0 {
            puzzle_year = parts;
        }
        if counter == 1 {
            puzzle_day = parts;
        }
        counter += 1
    }

    if puzzle_year == "22" {
        match puzzle_day {
            "1" => {
                year21::day1::part1();
                year21::day1::part2()
            }
            _ => println!("Doesn't exist"),
        }
    } else {
        println!("Doesn't exist")
    }
}
