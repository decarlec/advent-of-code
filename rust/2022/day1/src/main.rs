use std::{fs::File, io::BufReader, io::prelude::*};

fn main() {
    calc();
}

fn calc()  {

    //read elf list
    let file = File::open("cals.txt").unwrap();
    let reader = BufReader::new(file);
    let lines_iter = reader.lines().map(|l| l.unwrap());
    let mut elves : Vec<i32> = Vec::new();

    
    let mut sum : i32 = 0;
    for line in lines_iter {
        //Add elf to list
        if line.len() == 0{
            elves.push(sum);
            sum = 0;
        } else
        {
            //Add amount to elf
            let amt = line.parse::<i32>().unwrap();
            sum = sum + amt;  
        }
    }

    // find the elf kings
    let mut elf_kings : i32 = 0; 
    for _ in 0..3 {
        let (king, cals) = remove_king(&mut elves);
        elf_kings = elf_kings + cals;
        println!("{0}, is the elf king with {1} calories in his bag", king, cals); 
    }

    println!("There are a total of {} calories in the top 3 elves bags", elf_kings);
}

fn remove_king(elfs : &mut Vec<i32>) -> (usize, i32) {
    let mut king = 0;
    let mut king_idx : usize = 0;
    for (i, elf) in elfs.iter().enumerate() {
        if elf > &king {
            king = *elf;
            king_idx = i;
        }
    }
    elfs.remove(king_idx);
    (king_idx, king)
}




