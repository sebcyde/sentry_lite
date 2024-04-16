pub mod utils {
    use std::{ffi::OsString, path::PathBuf, usize};

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

        println!("RENAME: {:?} > {:?}\n", &file_path, &new_path);

        std::fs::rename(file_path, &new_path).expect("Failed to rename file");

        println!("Reanme complete\n");

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

    pub fn move_directory(
        current_dir_path: PathBuf,
        destination_dir: PathBuf,
    ) -> std::io::Result<()> {
        // TODO -> Add zipping functionality

        println!("ORIGINAL DIR DESTINATION ->> {:?}\n\n", &destination_dir);

        let new_dir_name: String = get_file_name_lower(&current_dir_path);
        println!("new_dir_name ->> {:?}", &new_dir_name);

        let destination: PathBuf = rename_file(&current_dir_path, &new_dir_name);
        println!("destination ->> {:?}", &destination);

        return std::fs::rename(current_dir_path, destination);
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
