use std::{fs::File, io::BufReader, io::prelude::*};
use std::str::FromStr;

enum Move {
    Rock,
    Paper,
    Scissors
}

enum Outcome {
    Win,
    Lose,
    Draw
}

struct Game {
    move1: Move,
    move2: Move
}

// just for fun
trait Scored {
    fn calc_score(&self) -> (i32, i32);
}


impl FromStr for Move {
    type Err = ();

    fn from_str(s: str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            _ => Err(()),
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines_iter = reader.lines().map(|l| l.unwrap());
    let mut p1_score = 0;
    let mut p2_score = 0;

    for line in lines_iter {
        let split : Vec<&str> = line.split_whitespace().collect();
        let move1 = Move::from_str(split[0]).unwrap();
        let out_come = Outcome::from_str(split[1]).unwrap(); 

        let move2 = calc_move(&move1, out_come);

    let game = Game { move1: move1, move2: move2 };
        let (p1_res, p2_res) = game.calc_score();
        p1_score += p1_res;
        p2_score += p2_res;
    }
    println!("Results \n Player 1: {0} pts \n Player 2: {1} pts ", p1_score, p2_score);
}

fn calc_move(opp : &Move, out : Outcome) -> Move {
    match out {
        Outcome::Win => {
            match opp {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock
            }
        },
        Outcome::Lose => {
            match opp {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper
            }
        }
        Outcome::Draw => {
            //TODO: fix this
            match opp {
                Move::Rock => Move::Rock,
                Move::Paper => Move::Paper,
                Move::Scissors => Move::Scissors
            }
        }
    }
}

impl Scored for Game {

    fn calc_score(&self) -> (i32, i32) {
        let mut p1 = 0;
        let mut p2 = 0;

        // +1 point for rock, 2 for paper, 3 for scissors
        match self.move1 {
            Move::Rock => p1 += 1,
            Move::Paper => p1 += 2,
            Move::Scissors => p1 += 3
        }

        match self.move2 {
            Move::Rock => p2 += 1,
            Move::Paper => p2 += 2,
            Move::Scissors => p2 += 3
        }

        // +6 for win, 3 for draw, 0 for loss
        match self.move1 {
            Move::Rock => 
            {
                match self.move2 {
                    Move::Rock => {
                        p1 += 3;
                        p2 += 3;
                    },
                    Move::Paper => {
                        p2 += 6
                    }
                    Move::Scissors => {
                        p1 += 6;
                    },
                }
            },
            Move::Paper => 
            {
                match self.move2 {
                    Move::Paper => {
                        p1 += 3;
                        p2 += 3;
                    },
                    Move::Scissors => {
                        p2 += 6
                    }
                    Move::Rock => {
                        p1 += 6;
                    },
                }
            },
            Move::Scissors => 
            {
                match self.move2 {
                    Move::Scissors => {
                        p1 += 3;
                        p2 += 3;
                    },
                    Move::Rock => {
                        p2 += 6
                    }
                    Move::Paper => {
                        p1 += 6;
                    },
                }
            }
        }
        (p1, p2)
    }

}
