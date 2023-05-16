// Copyright (C) 2023-Present Divine Niiquaye Ibok.
// This file is part the of Yuniye Programming Language.

// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

#![feature(vec_into_raw_parts)]
#![feature(exitcode_exit_method)]

use std::{env, process};
use colored::Colorize;

#[derive(Debug)]
pub enum Command {
    Interactive,
    Run(String),
    Explain(String),
    New(String),
    AddSync,
    Remove,
    Build,
    Clean(String),
    Help,
}

impl Command {
    /// Language version is taken from Cargo.toml.
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    /// The Last tagged commit id.
    pub const LAST_COMMIT_ID: &str = "e1d3b9a";

    /// Release Type (Experimental, Beta, Nightly, Stable)
    pub const RELEASE_TYPE: &str = "Experimental";

    pub fn help() -> String {
        format!(
            "{}
{}
  yuniye [OPTIONS] [COMMAND] [args...]
  yuniye [OPTIONS] <file> Runs a Yuniye file (.yy or .yn)
  yuniye [OPTIONS] <folder> Runs the root file from a given folder

{}
  {}            Prints version info and exit
  {}                Run as interactive shell
  {}     Increase the verbosity of messages: 1 for normal output, 2 for more verbose output and 3 for debug
  {}              Do not output any message
  {}             Override a configuration value
  {}         Run with built-in web server.
  {}               Print help information

{}
  yuniye {}          Create a new Yuniye project
  yuniye {}            Installs/synchronizes dependencies
  yuniye {}              Removes installed dependencies
  yuniye {}               Installs dependencies and/or builds current project
  yuniye {}      Removes all compiled files from given folder

See `duke {}` for more information on a specific command.
            ",
            format!("\n{} version {} \n", "The Yuniye Programming Language".green().bold(), Self::VERSION.yellow()),
            "Usage:".yellow(),
            "Options:".yellow(),
            "-V, --version".green(),
            "-r, --run".green(),
            "-v|vv|vvv, --verbose".green(),
            "-q, --quite".green(),
            "-c, --config".green(),
            format!("-S <{}>:<{}>", "addr".cyan(), "port".magenta()).green(),
            "-h, --help".green(),
            "Project usage:".yellow(),
            format!("new <{}>", "name".cyan()).green(),
            "add/sync".green(),
            "remove".green(),
            "build".green(),
            format!("clean <{}>", "folder".cyan()).green(),
            format!("help <{}>", "command".cyan()).green(),
        )
    }

    pub fn version() -> String {
        let other = format!("({} {})", Self::LAST_COMMIT_ID.magenta(), Self::RELEASE_TYPE.white()).cyan();
        format!("{} {} {}", "Yuniye".green(), Self::VERSION.yellow(), other)
    }
}

fn main() {
    let mut args = env::args().skip(1);
    let command: Option<Command> = None;
    //let root = Path::new(&args.next().to_string()).parent().unwrap().display().to_string();

    while let Some(arg) = args.next() {
        match arg.as_ref() {
            "-V" | "--version" => {
                println!("{}", Command::version());
                process::ExitCode::SUCCESS.exit_process();
            },
            "-h" | "--help" => {
                println!("{}", Command::help());
                process::ExitCode::SUCCESS.exit_process();
            }
            "-r" | "--run" => {
                if command.is_some() {
                    panic!("The `--run` expects no command");
                }

                process::ExitCode::SUCCESS.exit_process();
            },
            "-t" => {
                let s = std::time::Instant::now();
                let tokens: Result<Vec<i32>, String> = Ok(vec![1, 2, 3]);
                let e = s.elapsed();

                match tokens {
                    Ok(tokens) => {
                        println!("{:?} tokens in {:?}", tokens.len(), e);

                        for i in tokens {
                            println!("{:?}", i);
                        }

                        process::ExitCode::SUCCESS.exit_process();
                    },
                    Err(_) => {
                        process::ExitCode::FAILURE.exit_process();
                    },
                }
            }
            arg => {
                if let Some(cmd) = command {
                    panic!("The `{:?}` expects no command, found {}", cmd, arg);
                }
            }
        }
    }

    println!("{}", Command::help());
    process::ExitCode::SUCCESS.exit_process();
}
