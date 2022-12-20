use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let priority = "0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        let mut sum_priorities = 0;
        let mut group: Vec<String> = vec!["".to_string(), "".to_string(), "".to_string()];

        for line in lines {
            if let Ok(x) = line {
                /*
                let half= x.len() / 2;
                let first_half =  &x[..half];
                let second_half = &x[half..];
                println!("{}, {}, {}, {}", x, x.len(), first_half, second_half );
                for c in first_half.chars() {
//                    println!("char: {}", c);
                    if let Some(i) = second_half.find(c) {
                        if let Some(p) = priority.find(c) {
                            println!("{c} is in both compartments, with priority {p}");
                            sum_priorities += p;
                        }
                        break;
                    }

                }
                 */
                if group[0] == "" {
                    group[0] = x;
                    continue;
                }
                if group[1] == "" {
                    group[1] = x;
                    continue;
                }
                if group[2] == "" {
                    group[2] = x;
                    let badge: char = find_badges(&group);
                    if let Some(p) = priority.find(badge) {
                        println!("Badge: {badge} with priority {p}"); 
                        sum_priorities += p;                       
                    }
                    group = vec!["".to_string(), "".to_string(), "".to_string()];
                }
            }
        }
        println!("Total of priorities: {sum_priorities} ");
    } else {
        println!("Unable to open file");
    }
}
fn find_badges(group: & Vec<String> ) -> char {
    let mut badge = '0';
    for c in group[0].chars() {        
        if let Some(i) = group[1].find(c) {
            if let Some(j) = group[2].find(c) {
                badge = c ;
            }
        }
    }
    badge
}





