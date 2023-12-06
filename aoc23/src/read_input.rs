use std::{
    fs::File,
    io::{prelude::*, BufReader},
 //   path::Path,
};
pub fn read_entire_file(path : &str) -> Vec<String>{
       let file = File::open(path).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
