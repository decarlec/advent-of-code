use std::io::{ BufReader, BufRead};
use std::iter::*;

struct File {
    path: Vec<String>,
    size: u32,
    children: Vec<File>,
    is_dir: bool
}

fn main () {
    let input = std::fs::File::open("input.txt").unwrap();
    let reader = BufReader::new(input);
    let lines_iter = reader.lines().map(|x| String::from(x.unwrap()));
    let mut current_path : Vec<String> = vec!["root".to_string()];
    
    let mut root = File { path: current_path.to_owned(), size: 0, children: Vec::new(), is_dir: true }; 
    let current = &root;

    for line in lines_iter {
        let sections : Vec<&str> = line.split_whitespace().collect();
        match sections[..] {
            ["$", "cd", dir] => { 
                match dir {
                    ".." => _ = current_path.pop(),
                    _ => current_path.push(dir.to_string())
                }
            }
            ["$", "ls"] => println!("ls command"),
            ["dir", dir] => println!("dir: {}", dir),
            [size, name] => println!("{}, size:{}", name, size),
            _ => println!("no matches!")
        }
    }
} 
