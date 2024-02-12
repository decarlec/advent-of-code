use std::{fs::File, io::BufReader, io::prelude::*};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines_iter = reader.lines().map(|l| l.unwrap());
    let mut count = 0;

    for line in lines_iter {
        println!("{}", line);
        let line_split : Vec<&str> = line.split(',').collect();
        let r1 : Vec<&str> = line_split[0].split('-').collect();
        let r2 : Vec<&str> = line_split[1].split('-').collect();

        let a = (r1[0].parse::<i32>().unwrap(), r1[1].parse::<i32>().unwrap());
        let b = (r2[0].parse::<i32>().unwrap(), r2[1].parse::<i32>().unwrap());

        // Check for any overlap
        if a.0 < b.0 {
            if a.1 >= b.0 { count += 1; } 
        }
        else {
            if b.1 >= a.0 { count += 1; }
        }
        
        // Check for containing overlap
        // if (a.0 <= b.0 && b.1 <= a.1) | (b.0 <= a.0 && a.1 <= b.1){
        //     count += 1;
        //
        // }
        println!("{}", count);
    }
}
