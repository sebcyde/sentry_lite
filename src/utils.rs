pub mod utils {
    use std::{
        ffi::{OsStr, OsString},
        path::PathBuf,
        usize,
    };

    use dialoguer::Select;

    use crate::actions::actions::{clean, kill, purge, watch};

    pub fn startup_prompt() {
        let options: Vec<&str> = vec!["Clean", "Watch", "Archive", "Kill", "Quit"];

        println!("\nClean - Initiates a single clean cycle");
        println!("Watch - Initiates watch mode with constant clean cycle");
        println!("Kill - Kills any currently running Sentry watch instances");
        println!("Quit - End the program\n");

        let selection: usize = Select::new()
            .with_prompt("Select an action")
            .default(0)
            .items(&options)
            .interact()
            .unwrap();

        let selected_option: &str = options[selection];
        println!("Selected: {}\n", selected_option);

        match selected_option.to_ascii_lowercase().as_str() {
            "purge" => purge(),
            "clean" => clean(),
            "watch" => watch(),
            "kill" => kill(),
            "quit" => std::process::exit(0),
            _ => panic!(),
        }
    }

    fn get_file_name(file_path: &PathBuf) -> String {
        return file_path
            .to_owned()
            .file_name()
            .unwrap()
            .to_owned()
            .into_string()
            .unwrap();
    }

    fn get_file_name_lower(file_path: &PathBuf) -> String {
        let file_name: OsString = file_path
            .to_owned()
            .file_name()
            .unwrap()
            .to_ascii_lowercase();

        return file_name.into_string().unwrap().replace(" ", "_");
    }

    fn rename_file(file_path: &PathBuf, new_file_name: &str) {
        let mut new_path: PathBuf = file_path.clone();
        new_path.pop();
        new_path.push(get_file_name_lower(&PathBuf::from(new_file_name)));

        std::fs::rename(file_path, new_path).expect("Failed to rename file");
    }

    pub fn zip_directory(_dir_path: PathBuf) {}
    pub fn move_file(current_file_path: PathBuf, destination_path: PathBuf) -> Option<PathBuf> {
        if !current_file_path.exists() {
            println!("Invalid path provided");
            return None;
        }

        if !current_file_path.is_file() {
            println!("Provided path is not a file.");
            return None;
        }

        // Format file name to lower + replace spaces
        let old_file_name: String = get_file_name(&current_file_path);
        let new_file_name: String = get_file_name_lower(&current_file_path);
        rename_file(&current_file_path, &new_file_name);
        println!("Format Complete: {:?} > {}", &old_file_name, &new_file_name);

        if destination_path.exists() {
            println!("An entity already exists at specified destination.");
        }

        // Move the file
        std::fs::rename(current_file_path, &destination_path).expect("Failed to rename file");
        println!("File move complete.");

        return Some(destination_path);
    }
}
