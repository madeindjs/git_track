extern crate colored;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use colored::*;

/// Open file & get Line to iterate on it
fn get_logs() -> std::io::Lines<BufReader<File>> {
    let log_file_path: &str = ".tickets_count.log";

    match File::open(log_file_path) {
        Ok(file) => {
            return BufReader::new(file).lines();
        }
        Err(_) => panic!("Can't open file"),
    };
}


fn main() {
    let mut counts_branchs: HashMap<String, u64> = HashMap::new();
    let mut max_space: usize = 0;

    for log in get_logs() {
        let branch: String = log.unwrap();
        // get number of log, if not exists: create a 0
        let count_branch: u64 = match counts_branchs.get(&branch) {
            Some(number) => number + 1,
            None => 0,
        };

        // get maximum lenght of branch to display prettier
        let branch_len = branch.len();
        if max_space < branch_len {
            max_space = branch_len;
        }

        // insert / update result
        counts_branchs.insert(branch, count_branch);
    }

    // display counts
    for (branch, count) in counts_branchs {
        println!(
            "\t{}{}  {:.01}",
            branch.bold(),
            // vertical align with spaces
            " ".repeat(max_space - branch.len()),
            (count as f64 / 60f64)
        );
    }
}
