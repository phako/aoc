use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Dir {
    size:u64,
    children: HashMap<String, Dir>,
    name:String,
}

impl Dir {
    fn new(name_: String) -> Self{
        Dir {
            size: 0,
            children: HashMap::new(),
            name : name_,
        }
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    let Ok(lines) = read_lines("input") else { return };

    // Consumes the iterator, returns an (Optional) String
    let mut stack:Vec<Dir> = Vec::new();
    let mut all_folders:Vec<Dir> = Vec::new();
    let mut sum:u64 = 0;
    for line in lines {
        if let Ok(ip) = line {
            let mut info = ip.split(" ");
            if let Some(first) = info.next() {
                match first {
                    "$" => {
                        let Some(command) = info.next() else { panic!("Wrong input")};
                        match command {
                            "ls" => {},
                            "cd" => {
                                let Some(name) = info.next() else { panic!("Wrong input!")};
                                if name == ".." {
                                    // Part one: collect all sizes below 100000
                                    let foo = stack.pop().unwrap();
                                    if foo.size <= 100000 {
                                        sum += foo.size;
                                    }
                                    stack.last_mut().unwrap().size += foo.size;

                                    // Part two: Collecting the folders here for later processing
                                    all_folders.push(foo);
                                } else {
                                    stack.push(Dir::new(name.to_string()));
                                    //println!("going into {} {:?}", name, stack);
                                }
                            },
                            _ => { panic!("Invalid input");}
                        }
                    },
                    "dir" => {
                        let Some(name) = info.next() else { panic!("Wrong input")};
                        //stack.last_mut().unwrap().children.insert(name.to_string(), Dir::new(name.to_string()));
                    },
                    _ => {
                        let Ok(value) = first.parse::<u64>() else { panic!{"Wrong input"}};
                        stack.last_mut().unwrap().size += value;
                    },
                }
            } else {
                panic!("Invalid input");
            }
        }
    }
    
    // Unwind left-over stack
    while stack.len() > 0 {
        let foo = stack.pop().unwrap();
        if foo.size <= 100000 {
            sum += foo.size;
        }

        if !stack.is_empty() {
            stack.last_mut().unwrap().size += foo.size;
        }
        all_folders.push(foo);
    }

    // Part two: Collect the minimal size we need to free
    let total_size = all_folders.last().unwrap().size;
    let space_left = 70000000 - total_size;
    let minimum_required = 30000000 - space_left;
    println!("Total Size: {}, Space left: {}, need an additional {}", total_size, space_left,  minimum_required);


    // Use that for find the smallest one that exceeds that size here 
    all_folders.sort_by(|a, b| a.size.cmp(&b.size));
    let Some(dir) = all_folders.iter().find(|dir| dir.size > minimum_required) else {panic!("No space left to free")};
    println!("{:?}", dir);

    println!("Puzzle output: {}", sum);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}