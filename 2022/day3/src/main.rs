use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn score(element:&char) -> u32 {
    match element {
        'a'..='z' => (*element as u32) - ('a' as u32) + 1,
        'A'..='Z' => (*element as u32) - ('A' as u32) + 27,
        _ => panic!("Invalid input {}", element),
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    let Ok(lines) = read_lines("example.txt") else { return };

    // Consumes the iterator, returns an (Optional) String
    let mut total_score:u32 = 0;
    for line in lines {
        if let Ok(ip) = line {
            let compartment_size = ip.len() / 2;
            let first = &ip[..compartment_size];
            let second = &ip[compartment_size..];
            let mut first_elements_unique = HashSet::new();
            first_elements_unique.extend(first.chars());
            let mut second_elements_unique = HashSet::new();
            second_elements_unique.extend(second.chars());

            let same:Vec<&char> = first_elements_unique.intersection(&second_elements_unique).collect();
            assert!(same.len() == 1);

            let first_element = same.get(0).unwrap();

            //println!("Frist: {}, second: {}, same element: {}, score: {}", first, second, first, score(first_element));
            total_score += score(first_element)
        }
    }
    println!("Total Score: {}", total_score);

    {
        total_score = 0;
        let Ok(lines) = read_lines("input") else { return };
        let mut line_counter:u32 = 0;
        let mut same = HashSet::new();
        for line in lines {
            if let Ok(ip) = line {
                let mut unique = HashSet::new();
                unique.extend(ip[..].chars());
                if same.is_empty() {
                    same.extend(ip[..].chars());
                }
                println!("{:?} -> {:?}", unique, same);

                let current = same.clone();

                let intersection = current.intersection(&unique);
                same.clear();
                same.extend(intersection);
                println!("{:?} -> {:?}", unique, same);

                if line_counter % 3 == 2 {
                    println!("Resetting after three elves: {}", line_counter);
                    assert!(same.len() == 1);
                    let foo:Vec<char> = same.drain().collect();
                    total_score += score(foo.get(0).unwrap())
                }

                //println!("Frist: {}, second: {}, same element: {}, score: {}", first, second, first, score(first_element));
            }
            println!();
            line_counter += 1;
        }
    }
    println!("Total Score: {}", total_score);

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}