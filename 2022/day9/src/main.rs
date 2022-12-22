use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct End {
    x: i64,
    y: i64,
}

impl End {
    fn new() -> Self {
        End {
            x: 0,
            y: 0,
        }
    }

    fn follow(&mut self, other:&End) {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        
        // On top of each other
        if dx == 0 && dy == 0 { return }

        // Same height
        if dy == 0 {
            // Next to each other. Nothing to do
            if dx.abs() == 1 { return }

            self.x += dx.signum();

            return;
        } 


        // Not on the same height, but same vertically
        if dx == 0 {
            // next to each other, nothing to do
            if dy.abs() == 1 { return }

            self.y += dy.signum();

            return;
        }

        // Just one away diagonally, nothing to do
        if (dx.abs() == 1 && dy.abs() == 1) { return }
        // Diagonally different, move towards head diagonally
        self.x += dx.signum();
        self.y += dy.signum();
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    let Ok(lines) = read_lines("input") else { return };

    let mut tails:HashSet<(i64, i64)> = HashSet::new();

    // Part1 : Use rope size of 2 instead of 10
    let mut rope = vec![End { x:0, y:0 }; 10];
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(ip) = line {
            let mut parts = ip.split(" ");
            let direction = parts.next().unwrap();
            let amount:i64 = parts.next().unwrap().parse().unwrap();
            match direction {
                "R" => {
                    for _ in 0..amount {
                        rope.first_mut().unwrap().x += 1;
                        for i in 1..rope.len() {
                            // Find a way to solve this without cloning?
                            let foo = rope[i-1].clone();
                            rope[i].follow(&foo);
                        }
                        tails.insert((rope.last().unwrap().x, rope.last().unwrap().y));
                    }
                }
                "L" => {
                    for _ in 0..amount {
                        rope[0].x -= 1;
                        for i in 1..rope.len() {
                            let foo = rope[i-1].clone();
                            rope[i].follow(&foo);
                        }
                        tails.insert((rope.last().unwrap().x, rope.last().unwrap().y));
                    }
                }
                "U" => {
                    for _ in 0..amount {
                        rope[0].y -= 1;
                        for i in 1..rope.len() {
                            let foo = rope[i-1].clone();
                            rope[i].follow(&foo);
                        }
                        tails.insert((rope.last().unwrap().x, rope.last().unwrap().y));
                    }
                }
                "D" => {
                    for _ in 0..amount {
                        rope[0].y += 1;
                        for i in 1..rope.len() {
                            let foo = rope[i-1].clone();
                            rope[i].follow(&foo);
                        }
                        tails.insert((rope.last().unwrap().x, rope.last().unwrap().y));
                    }
                }
                _ => { 
                    panic!("Invalid input, unknown direction {}", direction);
                }
            }
        }
    }

    println!("Tail visited {} places at least once", tails.len());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}