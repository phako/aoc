use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Elf {
    intake: u64,
}

impl fmt::Display for Elf {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.intake)
    }   
}

fn main() {
    // File hosts must exist in current path before this produces output
    let Ok(lines) = read_lines("input") else { return };

    // Consumes the iterator, returns an (Optional) String
    let mut elves:Vec<Elf> = vec![Elf { intake: 0}];
    for line in lines {
        if let Ok(ip) = line {
            if ip.is_empty() {
                elves.push(Elf { intake: 0});
            } else {
                if let Some(elf) = elves.last_mut() {
                    elf.intake += ip.parse::<u64>().unwrap();
                }
            }
        }
    }

    elves.sort_by(|a, b| b.intake.cmp(&a.intake));
    println!("{:?}", elves);

    if let Some(last) = elves.first() {
        println!("{}", last.intake);
    }

    // That can probably just be a for loop
    let mut total:u64 = 0;
    elves.iter().take(3).for_each(|elf| total += elf.intake);
    println!("{}", total);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}