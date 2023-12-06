mod read_input;
use crate::read_input::read_entire_file;

fn main() {
    day1_part1();
    day1_part2();
}

pub fn day1_part1() -> String{
    let mut sum = 0;
    //let lines = read_entire_file("../input/day1_ex.txt");
    let lines = read_entire_file("../input/day1.txt");
    for line in lines{
        let mut first = 0;
        let mut last = 0;
        let mut seen_first = false;

        for char in line.chars(){
            if char.is_numeric(){
                if !seen_first{
                    first = char.to_digit(10).expect("char to be a number");
                    seen_first = true;
                }
                last = char.to_digit(10).expect("char to be a number");
                //print!("{} : {}",char.is_numeric(),char);
            }
        }
        sum += first*10 + last;
    }
    println!("{}",sum);
    sum.to_string()
}


pub fn day1_part2() -> String{
    let mut sum = 0;
    //let lines = read_entire_file("../input/day1_ex.txt");
    let lines = read_entire_file("../input/day1.txt");
    for line in lines{
        let mut first = 0;
        let mut last = 0;
        let mut seen_first = false;

        for char in line.chars(){
            if char.is_numeric(){
                if !seen_first{
                    first = char.to_digit(10).expect("char to be a number");
                    seen_first = true;
                }
                last = char.to_digit(10).expect("char to be a number");
                //print!("{} : {}",char.is_numeric(),char);
            }
            else{
                todo!("Add functionality to check for worded numbers");
                //let parsed = check_for_worded_number(line.split_off
            }
        }
        sum += first*10 + last;
    }
    println!("{}",sum);
    sum.to_string()
}
