use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let Ok(lines) = read_lines("input") else { return };

    // Consumes the iterator, returns an (Optional) String
    let mut overlapping_pairs = 0;
    let mut partially_overlapping_pairs = 0;
    for line in lines {
        if let Ok(ip) = line {
            let ranges:Vec<&str> = ip.split(",").collect();
            let elf1:Vec<u32> = ranges[0].split("-").map(|x| x.parse().unwrap()).collect();
            let elf2:Vec<u32> = ranges[1].split("-").map(|x| x.parse().unwrap()).collect();

            // Need to increase the end by one because rust ranges are right-exclusive
            let elf1_range = Range { start: elf1[0], end: elf1[1] + 1};
            let elf2_range = Range { start: elf2[0], end: elf2[1] + 1};

            // Check totally contained things - both start and end need to be in there
            if (elf1_range.contains(&elf2[0]) && elf1_range.contains(&elf2[1])) ||
                (elf2_range.contains(&elf1[0]) && elf2_range.contains(&elf1[1])) {
                    overlapping_pairs += 1;
            }

            // Check partial overlap - either start or end needs to be in there
            if (elf1_range.contains(&elf2[0]) || elf1_range.contains(&elf2[1])) ||
                (elf2_range.contains(&elf1[0]) || elf2_range.contains(&elf1[1])) {
                    partially_overlapping_pairs += 1;
            }
        }
    }
    println!("Pairs that are totally contained in each other: {}, simple overlap pairs: {}", overlapping_pairs, partially_overlapping_pairs);

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}