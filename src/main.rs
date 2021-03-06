extern crate argparse;
extern crate colored;
extern crate git2;


use argparse::{ArgumentParser, Store, StoreTrue};
use colored::*;
use git2::Repository;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, BufRead, Write};
use std::collections::HashMap;
use std::env;
use std::process;

const LOG_FILEPATH : &str = ".git_track.log";

/// Simple object to store arguments passed to command line
struct Configuration {
    /// name of branch to delete
    delete : String,
    /// run this tool in daemon or not
    watch: bool,
}

/// Parse arguments passed to command line
fn set_argparse() -> Configuration{
    let mut configuration = Configuration{delete: "".to_string(), watch: false};
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Log branch activity each minutes to get time spent report.");
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

/// Remove line matching with given log name
fn remove_log(log_to_delete : String) {
    // store logs contained in folder
    let mut logs : Vec<String> = Vec::new();
    for log in get_logs() {
        let branch: String = log.unwrap();
        // filter log to remove specified branch
        if branch != log_to_delete {
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

/// Output a resume of time spent
fn display_resume() {
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

/// Watch this repository to store current branch each minutes
fn watch_repository() {
    loop {
        {// used to clean memory before
            let repo : Repository = initialize_repository();
            let branch : String = get_current_branch(&repo);
            // add row here
            let mut file = OpenOptions::new().append(true).open(LOG_FILEPATH).unwrap();
            let _ = file.write(format!("{}\r\n", branch).as_bytes());
        }
        std::thread::sleep(std::time::Duration::from_millis(60000));
    }
}

/// Open current Git repository
fn initialize_repository() -> Repository {
    match Repository::init(".") {
        Ok(repo) => repo,
        Err(_) => panic!("This is not a valid Git repository"),
    }
}

fn get_current_branch(repo : &Repository) -> String {
    match repo.head() {
        Ok(reference) => {
            if reference.is_branch() {
                match reference.name() {
                    Some(name) => return name.replace("refs/heads/", ""),
                    None => panic!("Can't find name"),
                }
            }
        },
        Err(_) => panic!("az"),
    }

    return "".to_string();
}

fn main() {
    let configuration : Configuration = set_argparse();

    if configuration.watch {
        watch_repository();
    }

    // delete branch if asked
    if !configuration.delete.is_empty() {
        remove_log(configuration.delete);
    }

    display_resume();
}
