use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum Weapons {
    Rock,
    Paper,
    Scissors
}

const WIN: u32 = 6;
const LOSS: u32 = 0;
const DRAW: u32 = 3;

impl fmt::Display for Weapons {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self {
            Weapons::Rock => write!(f, "Rock"),
            Weapons::Paper => write!(f, "Paper"),
            Weapons::Scissors => write!(f, "Scissors"),
        }
    }   
}
impl Weapons {
    fn fight(&self, other: &Weapons) -> u32 {
        match self {
            Weapons::Rock =>  {
                match other {
                    Weapons::Rock => return self.score() + DRAW,
                    Weapons::Paper => return self.score() + LOSS,
                    Weapons::Scissors => return self.score() + WIN,
                }
            },
            Weapons::Paper =>  {
                match other {
                    Weapons::Rock => return self.score() + WIN,
                    Weapons::Paper => return self.score() + DRAW,
                    Weapons::Scissors => return self.score() + LOSS,
                }
            }
            Weapons::Scissors =>  {
                match other {
                    Weapons::Rock => return self.score() + LOSS,
                    Weapons::Paper => return self.score() + WIN,
                    Weapons::Scissors => return self.score() + DRAW,
                }
            }
        }
    }
    
    fn score(&self) -> u32 {
        match self {
            Weapons::Rock => 1,
            Weapons::Paper => 2,
            Weapons::Scissors => 3,
        }
    }

    fn from_string(input :&str) -> Option<Weapons> {
        match input {
            "A" | "X" => Some(Weapons::Rock),
            "B" | "Y" => Some(Weapons::Paper),
            "C" | "Z" => Some(Weapons::Scissors),
            _ => None,
        }
    }

    fn parse(input :&str) -> Option<Weapons> {
        match input {
            "A" => Some(Weapons::Rock),
            "B" => Some(Weapons::Paper),
            "C" => Some(Weapons::Scissors),
            _ => None,
        }
    }

    fn answer(&self, other: &str) -> Option<Weapons>{
        match self {
            Weapons::Rock =>
                match other {
                    "X" => Some(Weapons::Scissors),
                    "Y" => Some(Weapons::Rock),
                    "Z" => Some(Weapons::Paper),
                    _ => None,
                },
            Weapons::Paper =>
                match other {
                    "X" => Some(Weapons::Rock),
                    "Y" => Some(Weapons::Paper),
                    "Z" => Some(Weapons::Scissors),
                    _ => None,
                }
            Weapons::Scissors =>
                match other {
                    "X" => Some(Weapons::Paper),
                    "Y" => Some(Weapons::Scissors),
                    "Z" => Some(Weapons::Rock),
                    _ => None,
                }
        }
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    let Ok(lines) = read_lines("input") else { return };

    // Consumes the iterator, returns an (Optional) String
    let mut score1:u32 = 0;
    let mut score2:u32 = 0;
    for line in lines {
        if let Ok(ip) = line {
            let strategy: Vec<&str> = ip.split(" ").collect();
            let Some(other) = Weapons::parse(strategy[0]) else { panic!("Invalid input {}", strategy[0])};
            let Some(me) = Weapons::from_string(strategy[1]) else { panic!("Invalid input {}", strategy[1])};
            let Some(me2) = other.answer(strategy[1]) else {panic!("Invalid input {}", strategy[1])};

            //println!("Fighting: other: {}, me: {}, score: {}", other, me, me.fight(&other));
            //println!("Fighting: other: {}, me2: {}, score: {}", other, me2, me2.fight(&other));

            score1 += me.fight(&other);
            score2 += me2.fight(&other);
        }
    }

    println!("Total Score: {}/{}", score1, score2);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}