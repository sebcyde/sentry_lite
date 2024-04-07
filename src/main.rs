use crate::actions::actions::{archive, clean, kill, watch};

pub mod actions;
pub mod utils;

fn main() {
    println!(" ");
    println!("Starting Sentry (Lite)...");

    let startup_args: Vec<String> = std::env::args().collect();

    if startup_args.len() > 1 {
        match startup_args[1].to_ascii_lowercase().as_str() {
            "archive" => archive(),
            "clean" => clean(),
            "watch" => watch(),
            "kill" => kill(),
            _ => {
                println!(" ");
                println!("Invalid argument given. Available commands are:");
                println!(" ");
                println!("clean - Initiates a single clean cycle");
                println!("watch - Initiates watch mode with constant clean cycle");
                println!("kill - Kills any currently running Sentry watch instances");
                println!(" ");
            }
        }
    } else {
        println!(" ");
        println!("No argument given. Available commands are:");
        println!(" ");
        println!("clean - Initiates a single clean cycle");
        println!("watch - Initiates watch mode with constant clean cycle");
        println!("kill - Kills any currently running Sentry watch instances");
        println!(" ");
    }
}
