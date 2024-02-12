use std::env::VarError;
use std::fmt::Debug;
use std::fs::File;
use std::io::{ BufReader, BufRead};
use std::str::FromStr;
use std::iter::*;

enum Command {
    Move(String),
    List,
}

impl FromStr for Command { 
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items : Vec<&str> = s.split(' ').collect();
        match items[1] {
            "ls" => Ok(Command::List),
            "cd" => Ok(Command::Move(items[2].to_string())),
            _ => Err(()),
        }
    }
}

// impl fmt::Debug for Dir {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Dir")
//          .field("name", &self.name)
//          
//          .finish()
//     }
// }

enum Output {
    Dir(String),
    File{ name: String, size: i32 }
}

#[derive(Debug)]
struct Dir {
    name: String,
    files: Option<Vec<Fil>>,
    parent: Option<Box<Dir>>,
    dirs: Option<Vec<Dir>>
}

#[derive(Debug)]
struct Fil {
    name: String, 
    size: i32,
}

fn  main() {
    build_fs();
}

fn set_current_dir(current: &mut Option<Box<Dir>>, path: &str){
    if let Some(dir) = current.as_mut() {
        match &dir.dirs {
            Some(dirs) => {
                if let Some(dir) = dirs.iter().find(|x| x.name == path){
                    //println!("dir already exists! moving to current dir {}", dir.name);
                    current = &mut Some(Box::new(*dir));
                }
            },
            None => println!("dirs doesn't exist {}", path)
        }
    }
    else {
        let prev_dir = current;
        current = &mut Some(Box::new(Dir { name: String::from(path), parent: *prev_dir, files: None, dirs: None   })) 
    }
}

fn build_fs() -> Dir {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines_iter = reader.lines().map(|l| l.unwrap());

    let dirs : &mut Vec<Dir> = &mut Vec::new();
    let mut current_dir : Option<Box<Dir>> = None;

    for line in lines_iter {
        //Commands
        if line.starts_with('$') {
            let cm = Command::from_str(&line).unwrap();
            match cm {
                //Move command
                Command::Move(loc) => {
                    println!("move command recieved: {}", loc );
                    match loc.as_str() {
                        ".." => { current_dir = current_dir.unwrap().parent },
                        _ => {
                            set_current_dir(&mut current_dir, &loc);

                            //TODO: Save reference to current dir so we can navigate
                            //the structure via reference. If dir exists cd will move
                            //into it, if not new will be created.
                            //be navigated. When navigating to a dir,
                            //if that dir already

                            dirs.push(Dir { name: String::from(&loc), dirs: Some(Vec::new()), files: Some(Vec::new()), parent: None }); 

                            dbg!(current_dir); 
                        }
                    } 
                },
                //Ls command
                Command::List => {println!("ls command recieved");},
                _ => panic!("invalid command: {}", line)  
            } 
        }
        else {

            let words : Vec<&str> = line.split(' ').collect();
            // Dir output
            if line.starts_with("dir") {

                let dir = Dir { name: String::from(words[1]), files: Some(Vec::new()), parent: None, dirs: Some(Vec::new()) }; 
                println!("dir read: {}", dir.name); 
            }
            // File output
            if let Ok(size) = words[0].parse::<i32>(){
                println!("file read: {}, size: {}", words[1], size);

            }
        }

    }
    Dir { name: String::from(""), parent: None, files: Some(vec![Fil { name: String::from("a"), size: 1 }]), dirs: Some(Vec::new())}
}
