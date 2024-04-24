use std::path::PathBuf;

use crate::{
    actions::actions::{clean, kill, purge, watch},
    config::config::check_config,
    locations::locations::get_config_dir,
    utils::utils::startup_prompt,
};

pub mod actions;
pub mod config;
pub mod locations;
pub mod types;
pub mod utils;

fn main() {
    println!("\nStarting Sentry (Lite)...");

    let use_default_pathing: bool = check_config();

    if use_default_pathing {
        println!("Config not set. Using default pathing.\n");
    } else {
        println!("Config loaded succesfully.\n");
    }

    // panic!();

    let startup_args: Vec<String> = std::env::args().collect();

    if startup_args.len() > 1 {
        match startup_args[1].to_ascii_lowercase().as_str() {
            "purge" => purge(),
            "clean" => {
                clean(dirs::download_dir().unwrap());
            }
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
