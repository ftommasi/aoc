use std::{
    fs::File,
    io::{prelude::*, BufReader}, error,
    //   path::Path,
};
pub fn read_entire_file(path : &str) -> Vec<String>{
    let file = File::open(path).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
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

fn get_str_slice(s : &str, start : usize, end : usize) -> Option<&str>{
    if end > start && s.len() >= end {
        return Some(&s[start..end])
    }
    None
}

pub fn check_for_worded_number(line : String) -> Result<u32,&'static str>{
    match line.chars().nth(0){
        Some(x) => {
            match x {
                //one
                'o' => {
                    let maybe_name = get_str_slice(line.as_str(), 0, 3);
                    match maybe_name{
                        Some(name) =>{
                            match name {
                                "one" => {return Ok(1)},
                                _ => return Err("no matches")
                            }
                        },
                        None =>{
                            return Err("Out of bounds")
                        }
                    }
                },
                //two/three
                't' => {
                    match line.chars().nth(1){
                        Some(x) => {
                            match x{
                                'w' => { //two
                                    let maybe_name = get_str_slice(line.as_str(), 0, 3);
                                    match maybe_name{
                                        Some(name) =>{
                                            match name {
                                                "two" => {return Ok(2)},
                                                _ => return Err("no matches")
                                            }
                                        },
                                        None =>{
                                            return Err("Out of bounds")
                                        }
                                    }
                                },
                                'h' =>{ //three
                                    let maybe_name = get_str_slice(line.as_str(), 0, 5);
                                    match maybe_name{
                                        Some(name) =>{
                                            match name {
                                                "three" => {return Ok(3)},
                                                _ => return Err("no matches")
                                            }
                                        },
                                        None =>{
                                            return Err("Out of bounds")
                                        }
                                    }
                                },
                                _ => {
                                    return Err("no matches")
                                }
                            }
                        }
                        None =>{
                            return Err("string out of bounds")
                        }
                    }
                } ,
                //four/five
                'f' => {
                    match line.chars().nth(1){
                        Some(x) => {
                            match x{
                                'o' => { //four
                                    let maybe_name = get_str_slice(line.as_str(), 0, 4);
                                    match maybe_name{
                                        Some(name) =>{
                                            match name {
                                                "four" => {return Ok(4)},
                                                _ => return Err("no matches")
                                            }
                                        },
                                        None =>{
                                            return Err("Out of bounds")
                                        }
                                    }
                                },
                                'i' =>{ //five
                                    let maybe_name = get_str_slice(line.as_str(), 0, 4);
                                    match maybe_name{
                                        Some(name) =>{
                                            match name {
                                                "five" => {return Ok(5)},
                                                _ => return Err("no matches")
                                            }
                                        },
                                        None =>{
                                            return Err("Out of bounds")
                                        }
                                    }
                                },
                                _ => {
                                    return Err("no matches")
                                }
                            }
                        }
                        None =>{
                            return Err("string out of bounds")
                        }
                    }
                } ,
                //six/seven
                's' => {
                    match line.chars().nth(1){
                        Some(x) => {
                            match x{
                                'i' => { //six
                                    let maybe_name = get_str_slice(line.as_str(), 0, 3);
                                    match maybe_name{
                                        Some(name) =>{
                                            match name {
                                                "six" => {return Ok(6)},
                                                _ => return Err("no matches")
                                            }
                                        },
                                        None =>{
                                            return Err("Out of bounds")
                                        }
                                    }
                                },
                                'e' =>{ //seven
                                    let maybe_name = get_str_slice(line.as_str(), 0, 5);
                                    match maybe_name{
                                        Some(name) =>{
                                            match name {
                                                "seven" => {return Ok(7)},
                                                _ => return Err("no matches")
                                            }
                                        },
                                        None =>{
                                            return Err("Out of bounds")
                                        }
                                    }
                                },
                                _ => {
                                    return Err("no matches")
                                }
                            }
                        }
                        None =>{
                            return Err("string out of bounds")
                        }
                    }
                } ,
                //eight
                'e' => {
                    let maybe_name = get_str_slice(line.as_str(), 0, 5);
                    match maybe_name{
                        Some(name) =>{
                            match name {
                                "eight" => {return Ok(8)},
                                _ => return Err("no matches")
                            }
                        },
                        None =>{
                            return Err("Out of bounds")
                        }
                    }
                } ,
                //nine
                'n' => {
                    let maybe_name = get_str_slice(line.as_str(), 0, 4);
                    match maybe_name{
                        Some(name) =>{
                            match name {
                                "nine" => {return Ok(9)},
                                _ => return Err("no matches")
                            }
                        },
                        None =>{
                            return Err("Out of bounds")
                        }
                    }
                } ,
                _ => return Err("no number match starts here")
            }

        },
        None =>{
            return Err("string out of bounds")
        }
    }
}

pub fn day1_part2() -> String{
    let mut sum = 0;
    //let lines = read_entire_file("../input/day1_ex2.txt");
    let lines = read_entire_file("../input/day1.txt");
    for line in lines{
        let mut first = 0;
        let mut last = 0;
        let mut seen_first = false;

        for char in line.char_indices(){
            if char.1.is_numeric(){
                if !seen_first{
                    first = char.1.to_digit(10).expect("char to be a number");
                    seen_first = true;
                }
                last = char.1.to_digit(10).expect("char to be a number");
                //print!("{} : {}",char.is_numeric(),char);
            }
            else{
                //we clone because we may exit early and need the rest of the line.
                // perchance that is not needed. I am writing this tired and not caffeinated 
                let rest_of_line = line.clone().split_off(char.0);
                match check_for_worded_number(rest_of_line){
                    Ok(x) =>{
                        //println!("matched word {}", x);
                        if !seen_first{
                            first = x;
                            seen_first = true;
                        }
                        last = x;
                    },
                    Err(_x) =>{
                        //we might care later for debugging why it failed. ignore for now

                    }
                }
                //todo!("Add functionality to check for worded numbers");
                //let parsed = check_for_worded_number(line.split_off
            }
        }
        //println!("adding: {}", first*10 + last);
        sum += first*10 + last;
    }
    println!("{}",sum);
    sum.to_string()
}
