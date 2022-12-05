use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Mode {
    STACK,
    MOVES,
}

struct ElfStack {
    columns: Vec<VecDeque<char>>,
}

impl ElfStack {
    fn new() -> Self { ElfStack {
        columns: Vec::new(),
    } }

    fn push(&mut self, line: &str) {
        let create_columns = self.columns.len() == 0;
        let mut foo = line.chars().peekable();
        // iterate through the string in chunks of three
        // This might be easier with nightly and arra_chunks or sth.
        let mut column = 0;
        while foo.peek().is_some() {
            let chunk:Vec<char> = foo.by_ref().take(4).collect();
            if create_columns {
                self.columns.push(VecDeque::new())
            }
            if chunk[1].is_ascii_uppercase() {
                self.columns[column].push_front(chunk[1]);
            }
            column += 1;
        }
    }

    // Part one, move count elements one by one
    fn move_stacks(&mut self, line: &str){
        let mut chunks = line.split(" ");
        let count: usize = chunks.nth(1).unwrap().parse().unwrap();
        let source: usize = chunks.nth(1).unwrap().parse().unwrap();
        let dest: usize = chunks.nth(1).unwrap().parse().unwrap();

        for _ in 0..count {
            if let Some(top) = self.columns[source - 1].pop_back() {
                self.columns[dest - 1].push_back(top);
            }
        }
    }

    // part 2, move count elements as is
    fn move_stacks_en_block(&mut self, line: &str){
        let mut chunks = line.split(" ");
        let count: usize = chunks.nth(1).unwrap().parse().unwrap();
        let source: usize = chunks.nth(1).unwrap().parse().unwrap();
        let dest: usize = chunks.nth(1).unwrap().parse().unwrap();

        let len = self.columns[source - 1].len();

        let range = len-count..;

        //println!("Moving {} elements from {} with len {} to {} (using range {:?})", count, source, len, dest, range);

        // Collect to give up the mutable borrow on self.columns
        let mut tail: VecDeque<char> = self.columns[source - 1].drain(range).collect();
        self.columns[dest - 1].append(&mut tail);
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    let Ok(lines) = read_lines("input") else { return };

    // Consumes the iterator, returns an (Optional) String
    let mut mode = Mode::STACK;
    let mut stack = ElfStack::new();
    for line in lines {
        if let Ok(ip) = line {
            if ip.is_empty() {
                mode = Mode::MOVES;
                continue;
            }
            match mode {
                Mode::MOVES => stack.move_stacks_en_block(&ip),
                Mode::STACK => {
                    stack.push(&ip);
                },
            }
        }
    }
    for foo in &stack.columns {
        print!("{}", foo.back().unwrap());
    }
    println!();
    //println!("{:?}", stack.columns);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}