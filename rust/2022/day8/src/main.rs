use std::{fs::File, io::BufReader, io::prelude::*};
use std::str::FromStr;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines_iter = reader.lines().map(|l| l.unwrap());
    let mut p1_score = 0;
    let mut p2_score = 0;

    for line in lines_iter {
    }
}
