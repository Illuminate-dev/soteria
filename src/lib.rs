use std::io::{Read, Write};

use clap::Parser;
mod args;
mod app;

type Res = Result<(), ()>;

pub fn run() {
    let args = args::Args::parse();
    let app = app::App::new(args);
    app.main_screen()
}

fn input() -> String {
    let mut s = String::new();

    std::io::stdout().flush().expect("failed to flush stdout");
    std::io::stdin().read_line(&mut s).expect("error reading input");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    s
}