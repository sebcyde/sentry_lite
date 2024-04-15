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

        let downloads_directory: std::path::PathBuf = dirs::download_dir().unwrap();

        match selected_option.to_ascii_lowercase().as_str() {
            "purge" => purge(),
            "clean" => clean(downloads_directory),
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

        println!("RENAME: {:?} > {:?}\n", &file_path, &new_path);

        std::fs::rename(file_path, new_path).expect("Failed to rename file");

        println!("Reanme complete\n");
    }

    pub fn zip_directory(_dir_path: PathBuf) {}

    pub enum FILETYPE {
        IMAGE,
        VIDEO,
        AUDIO,
        DOCUMENT,
        DESIGN,
        ARCHIVE,
        CODE,
    }

    pub fn move_file(current_file_path: PathBuf, file_type: FILETYPE) -> Option<PathBuf> {
        let mut destination_path: PathBuf = match file_type {
            FILETYPE::IMAGE => dirs::picture_dir().unwrap(),
            FILETYPE::AUDIO => dirs::audio_dir().unwrap(),
            FILETYPE::DESIGN => dirs::picture_dir().unwrap(),
            FILETYPE::DOCUMENT => dirs::document_dir().unwrap(),
            FILETYPE::VIDEO => dirs::video_dir().unwrap(),
            FILETYPE::ARCHIVE => dirs::document_dir().unwrap(),
            FILETYPE::CODE => dirs::document_dir().unwrap(),
        };

        println!("Moving file to {:?}", &destination_path);

        // Format file name to lower + replace spaces
        let old_file_name: String = get_file_name(&current_file_path);
        let new_file_name: String = get_file_name_lower(&current_file_path);
        rename_file(&current_file_path, &new_file_name);

        println!("Format Complete: {:?} > {}", &old_file_name, &new_file_name);

        destination_path.push(&new_file_name);
        let mut old_path = current_file_path.clone();
        old_path.pop();
        old_path.push(&new_file_name);

        println!("DESTINATION: {:?}", &destination_path);

        // panic!();
        if destination_path.exists() {
            println!("An entity already exists at specified destination.\n");
            return None;
        } else {
            // Move the file
            std::fs::rename(&old_path, &destination_path).expect("Failed to rename file");
            println!("File move complete.\n");

            return Some(destination_path);
        }
    }
}
