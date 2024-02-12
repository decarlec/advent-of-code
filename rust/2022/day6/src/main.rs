use std::{fs::File, io::BufReader, io::prelude::*};

fn main() {

    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    //println!("{}", &line[1540..1544]);

    for idx in 0..line.len() {
        let sl = &line[idx..idx+14];
        if !check_duplicates(sl) {
            println!("marker found at {}", idx);
            return;
        };
    }
    
}

fn check_duplicates(sl : &str) -> bool {
    let mut seen : Vec<char> = Vec::new();
    for item in sl.chars() {
        if seen.contains(&item){ 
            return true;
        }
        seen.push(item);
    }
    println!("marker found! {}", sl);
    return false;
}
