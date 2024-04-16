pub mod utils {
    use std::{ffi::OsString, fs::ReadDir, path::PathBuf, usize};

    use dialoguer::Select;

    use crate::{
        actions::actions::{clean, kill, purge, watch},
        config::config::get_config_data,
        types::types::{UserConfig, FILETYPE},
    };

    pub fn exit_prompt(content: &str) {
        println!("\n{}\n", content);
        std::process::exit(0)
    }

    pub fn startup_prompt() {
        let options: Vec<&str> = vec!["Clean", "Watch", "Kill", "Quit"];

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
            "quit" | "q" | "c" | "exit" => std::process::exit(0),
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

    fn rename_file(file_path: &PathBuf, new_file_name: &str) -> PathBuf {
        let mut new_path: PathBuf = file_path.clone();
        new_path.pop();
        new_path.push(get_file_name_lower(&PathBuf::from(new_file_name)));
        std::fs::rename(file_path, &new_path).expect("Failed to rename file");
        return new_path;
    }

    // fn zip_directory(directory: PathBuf) -> Option<Pathbuf> {
    //     if !std::path::Path::is_dir(&directory) {
    //         return None;
    //     } else {
    //         let path = Path::new(&directory);
    //         let zip_file: &str = path.file_name().unwrap();

    //         let file = fs::File::create(zip_file)?;
    //         let mut zip = ZipWriter::new(file);
    //         let options = FileOptions::default()
    //             .compression_method(Stored)
    //             .unix_permissions(0o755);

    //         let mut buffer = Vec::new();
    //         let mut walkdir = fs::read_dir(directory)?;

    //         for entry in walkdir.by_ref() {
    //             let entry = entry?;
    //             let path = entry.path();
    //             let name = path.strip_prefix(directory)?.to_str().unwrap();

    //             if entry.file_type()?.is_file() {
    //                 zip.start_file(name, options)?;
    //                 let mut file = fs::File::open(path)?;
    //                 io::copy(&mut file, &mut zip)?;
    //             }
    //         }

    //         zip.finish()?;
    //         Some(path)
    //     }
    // }

    pub fn remove_dir_if_empty(destination_dir: &PathBuf) -> Option<&PathBuf> {
        let entries: ReadDir = std::fs::read_dir(&destination_dir).unwrap();
        let is_empty: &bool = &entries.count().eq(&0);

        if *is_empty {
            println!("is_empty: {:?}", &is_empty);

            _ = std::fs::remove_dir(&destination_dir);
            return None;
        }

        return Some(destination_dir);
    }

    pub fn move_directory(
        current_dir_path: PathBuf,
        destination_dir: PathBuf,
    ) -> std::io::Result<()> {
        // TODO -> Add zipping functionality

        let res: Option<&PathBuf> = remove_dir_if_empty(&current_dir_path);
        if res.is_none() {
            println!("Removed empty directory.");
            return Ok(());
        }

        println!("original destination:: {:?}", &destination_dir);

        let new_dir_name: String = get_file_name_lower(&current_dir_path);
        let mut new_destination: PathBuf = destination_dir.clone();

        new_destination.push(new_dir_name);

        println!("destination:: {:?}", &new_destination);

        return std::fs::rename(current_dir_path, new_destination);
    }

    pub fn move_file(current_file_path: PathBuf, file_type: FILETYPE) -> Option<PathBuf> {
        let user_config: UserConfig = get_config_data();

        let mut destination_path: PathBuf = match file_type {
            FILETYPE::DOCUMENT => PathBuf::from(user_config.documents_location),
            FILETYPE::ARCHIVE => PathBuf::from(user_config.archive_location),
            FILETYPE::FOLDERS => PathBuf::from(user_config.folders_location),
            FILETYPE::DESIGN => PathBuf::from(user_config.design_location),
            FILETYPE::IMAGE => PathBuf::from(user_config.image_location),
            FILETYPE::AUDIO => PathBuf::from(user_config.audio_location),
            FILETYPE::VIDEO => PathBuf::from(user_config.video_location),
            FILETYPE::CODE => PathBuf::from(user_config.code_location),
            FILETYPE::MISC => PathBuf::from(user_config.misc_location),
        };

        if destination_path.to_str().unwrap().eq_ignore_ascii_case("") {
            let mut misc_dir_raw: PathBuf = dirs::document_dir().unwrap();
            misc_dir_raw.push("Sentry");
            misc_dir_raw.push("Misc_Files");

            std::fs::create_dir_all(&misc_dir_raw).expect("Failed to create misc directory.");

            destination_path = misc_dir_raw;
            println!("\n\nFILE SKIPPED -- MOVED TO MISC DIR\n\n");
        }

        println!("Moving file to {:?}", &destination_path);

        // Format file name to lower + replace spaces
        let old_file_name: String = get_file_name(&current_file_path);
        let new_file_name: String = get_file_name_lower(&current_file_path);
        rename_file(&current_file_path, &new_file_name);

        println!("destination_path 1: {:?}", &destination_path);

        destination_path.push(&new_file_name);

        println!("destination_path 2: {:?}", &destination_path);

        let mut old_path = current_file_path.clone();

        println!("old_path 1: {:?}", &old_path);

        old_path.pop();

        println!("old_path 2: {:?}", &old_path);

        old_path.push(&new_file_name);
        println!("old_path 3: {:?}", &old_path);

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
