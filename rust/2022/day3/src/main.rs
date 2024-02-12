use std::{fs::File, io::BufReader, io::prelude::*};
use std::str::*;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines_iter = reader.lines().map(|l| l.unwrap());

    let mut total = 0;

    let mut elves : Vec<String> = Vec::new();

    for (idx, line) in lines_iter.enumerate() {
        // cannot handle odd numbered lines

        let line_copy = line as String;
        elves.push(line_copy);

        if elves.len() == 3 {
            let char = check_dups(&elves);
            println!("{}, {}, {}",elves[0], elves[1], elves[2]);
            //println!("{}", char); 
            total += get_prio(char);
            elves.clear(); 
        }


        // let a = &line[0..line.len()/2];
        // let b = &line[line.len()/2..line.len()];

        // let i_pri = get_prio(i);
        // let j_pri = get_prio(j);

        //total += i_pri; 
        //println!("1:{0} pri:{1}, 2:{2} pri:{3}", i, i_pri,  j, j_pri );

    }
    println!("Total prio: {}", total);
}


fn check_dups(elves : &Vec<String>) -> char {
    let mut a_and_b : Vec<char> = Vec::new();
    for i in elves[0].chars() {
        for j in elves[1].chars() {
            if i == j && !a_and_b.contains(&i){
                a_and_b.push(i);
                println!("{}", i);
            }           
        }
    }
    let elf_3 = elves[2].chars();
    for k in elf_3 {
        if a_and_b.iter().any(|x| x == &k) {
            return k;
        }
    }

    panic!("did not contain a duplicate!");
}

fn get_prio(c : char) -> u32 {
    let num = c as u32;
    match num {
        97..=122 => { return num - 96 },
        65..=90 => { return num - 38 },
        _ => ()
    }
    0 
}
