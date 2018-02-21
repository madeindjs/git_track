extern crate argparse;
extern crate colored;

use argparse::{ArgumentParser, Store};
use colored::*;
use std::fs::File;
use std::io::{BufReader, BufRead, Write};
use std::collections::HashMap;
use std::env;
use std::process;

struct Configuration {
    delete : String
}

fn set_argparse() -> Configuration{

    let mut configuration = Configuration{delete: "".to_string()};
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("will use a crontab & Git to log wich current branch you work on..");
        ap.refer(&mut configuration.delete)
            .add_option(&["-d", "--delete"], Store,
            "Delete given branch");
        ap.parse_args_or_exit();
    }

    return configuration;
}

/// Open file & get Line to iterate on it
fn get_logs() -> std::io::Lines<BufReader<File>> {
    let log_file_path: &str = ".tickets_count.log";

    match File::open(log_file_path) {
        Ok(file) => {
            return BufReader::new(file).lines();
        }
        Err(_) => {
            println!("File '.tickets_count.log' was not found");
            println!("you should install this crontab");
            println!(
                "\t* * * * * cd {0} && git branch 2> /dev/null | sed -e '/^[^*]/d' -e 's/* \\(.*\\)/\\1/' >> {0}/.tickets_count.log",
                env::current_dir().unwrap().display()
            );
            process::exit(1);
        },
    };
}


fn main() {
    let configuration = set_argparse();

    // delete branch if asked
    if !configuration.delete.is_empty() {
        // store logs contained in folder
        let mut logs : Vec<String> = Vec::new();
        for log in get_logs() {
            let branch: String = log.unwrap();
            // filter log to remove specified branch
            if branch != configuration.delete {
                logs.push(branch);
            }
        }
        // open file in write mode (clean file)
        let mut file = match File::create(".tickets_count.log") {
            Ok(file) => file,
            Err(_) => panic!("Can't open '.tickets_count.log' file, check access rights"),
        };

        for log in logs {
            let _ = file.write_all(format!("{}\r\n", log).as_bytes());
        }
    }

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
