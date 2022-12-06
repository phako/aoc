use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Scanner {
    size: usize,
    scanner: VecDeque<char>,
    found: bool,
    updates: usize,
}

impl Scanner {
    fn new(window:usize) -> Self{
        Scanner {
            size: window,
            scanner : VecDeque::new(),
            found : false,
            updates : 0,
        }
    }

    fn update(&mut self, input:char) -> bool {
        if self.found {
            return self.found;
        }

        self.updates += 1;

        let seen = self.scanner.iter().position(|&r| r == input);
        self.scanner.push_back(input);
        if let Some(index) = seen {
            self.scanner.drain(..index + 1);
        } else {
            self.found = self.scanner.len() == self.size;
        }

        self.found
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    let Ok(lines) = read_lines("input") else { return };

    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(ip) = line {
            let mut sop = Scanner::new(4);
            let mut som = Scanner::new(14);
            for c in ip.chars() {
                sop.update(c);
                som.update(c);
                if sop.found && som.found {
                    break;
                }
            }
            println!("SOP@{}, SOM@{}", sop.updates, som.updates);
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}