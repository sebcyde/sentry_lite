use crate::actions::actions::{clean, kill, watch};

pub mod actions;
pub mod utils;

fn main() {
    println!(" ");
    println!("Starting Sentry (Lite)...");

    let startup_args: Vec<String> = std::env::args().collect();

    if startup_args.len() > 1 {
        match startup_args[1].to_ascii_lowercase().as_str() {
            "clean" => clean(),
            "watch" => watch(),
            "kill" => kill(),
            _ => {
                println!("Invalid argument given. Available commands are:");
                println!("clean");
                println!("watch");
                println!("kill");
            }
        }
    }
}
