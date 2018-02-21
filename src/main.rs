extern crate argparse;
extern crate colored;
extern crate schedule;


use argparse::{ArgumentParser, Store, StoreTrue};
use colored::*;
use schedule::{Agenda, Job};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, BufRead, Write};
use std::collections::HashMap;
use std::env;
use std::process;

const LOG_FILEPATH : &str = ".git_track.log";

struct Configuration {
    delete : String,
    watch: bool,
}

fn set_argparse() -> Configuration{

    let mut configuration = Configuration{delete: "".to_string(), watch: false};
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("will use a crontab & Git to log wich current branch you work on..");
        ap.refer(&mut configuration.delete).add_option(&["-d", "--delete"], Store, "Delete given branch");
        ap.refer(&mut configuration.watch).add_option(&["-w", "--watch"], StoreTrue, "Watch this repository");
        ap.parse_args_or_exit();
    }

    return configuration;
}

/// Open file & get Line to iterate on it
fn get_logs() -> std::io::Lines<BufReader<File>> {
    match File::open(LOG_FILEPATH) {
        Ok(file) => {
            return BufReader::new(file).lines();
        }
        Err(_) => {
            println!("File '{}' was not found", LOG_FILEPATH);
            println!("you should install this crontab");
            println!(
                "\t* * * * * cd {0} && git branch 2> /dev/null | sed -e '/^[^*]/d' -e 's/* \\(.*\\)/\\1/' >> {0}/{1}",
                env::current_dir().unwrap().display(),
                LOG_FILEPATH
            );
            process::exit(1);
        },
    };
}



fn main() {
    let configuration = set_argparse();

    if configuration.watch {
        let mut a = Agenda::new();
        // Run every second
        a.add(Job::new(|| {
            // add row here
            let mut file = OpenOptions::new().append(true).open(LOG_FILEPATH).unwrap();
            let _ = file.write(b"test\r\n");
        }, "* * * * * *".parse().unwrap()));
        loop {
            a.run_pending();
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }


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
        let mut file = match File::create(LOG_FILEPATH) {
            Ok(file) => file,
            Err(_) => panic!(format!("Can't open '{}' file, check access rights", LOG_FILEPATH)),
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
