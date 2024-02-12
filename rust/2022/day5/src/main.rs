use std::{fs::File, io::BufReader, io::prelude::*};

fn main() {
    let mut shipyard = Shipyard {
        stacks:
            [
            vec!['H', 'R', 'B', 'D', 'Z', 'F', 'L', 'S'],
            vec!['T', 'B', 'M', 'Z', 'R'],
            vec!['Z', 'L', 'C', 'H', 'N', 'S'],
            vec!['S', 'C', 'F', 'J'],
            vec!['P', 'G', 'H', 'W', 'R', 'Z', 'B'],
            vec!['V', 'J', 'Z', 'G', 'D', 'N', 'M', 'T'],
            vec!['G', 'L', 'N', 'W', 'F', 'S', 'P', 'Q'],
            vec!['M', 'Z', 'R'],
            vec!['M', 'C', 'L', 'G', 'V', 'R', 'T']
            ]
    };

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines_iter = reader.lines().map(|l| l.unwrap());

    for line in lines_iter {
        let mv = parse_move(&line);    
            shipyard.move_boxes(mv.from, mv.to, mv.count);
    }

    for mut stack in shipyard.stacks {
        println!("{}", stack.pop().unwrap());
    }
}

struct Shipyard {
    stacks: [Vec<char>; 9]
}

struct Move {
    count: usize,
    from: usize,
    to: usize
}

fn parse_move(line : &str) -> Move {
    let str_split : Vec<&str> = line.split(' ').collect();
    Move { 
        count: str_split[1].parse::<usize>().unwrap(), 
        from: str_split[3].parse::<usize>().unwrap(), 
        to: str_split[5].parse::<usize>().unwrap()
    }
}

impl Shipyard {

    fn move_box(&mut self, from : usize, to : usize){
        let bx = self.stacks[from - 1].pop().unwrap();
        self.stacks[to - 1].push(bx);
    }

    fn move_boxes(&mut self, from : usize, to : usize, count: usize){
        let l = &self.stacks[from - 1].len();
        let mut bx = self.stacks[from - 1].split_off(l - count);
        self.stacks[to - 1].append(&mut bx);
    }
}
