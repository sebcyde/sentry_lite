use crate::{
    actions::actions::{clean, kill, purge, watch},
    utils::utils::startup_prompt,
};

pub mod actions;
pub mod utils;

fn main() {
    println!("\nStarting Sentry (Lite)...");

    let startup_args: Vec<String> = std::env::args().collect();

    if startup_args.len() > 1 {
        match startup_args[1].to_ascii_lowercase().as_str() {
            "purge" => purge(),
            "clean" => clean(),
            "watch" => watch(),
            "kill" => kill(),
            _ => {
                println!("\nInvalid argument given. Available commands are:");
                startup_prompt();
            }
        }
    } else {
        println!("\nNo argument given. Available commands are:");
        startup_prompt();
    }
}
