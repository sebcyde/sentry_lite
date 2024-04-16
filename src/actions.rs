pub mod actions {
    use std::{fs::read_dir, path::PathBuf};

    use sysinfo::{Pid, Process, System};

    use crate::{
        config::config::get_config_data,
        types::types::{UserConfig, FILETYPE},
        utils::utils::{move_directory, move_file},
    };

    pub fn clean(directory: PathBuf) {
        println!("\nCleaning...");
        let config: UserConfig = get_config_data();

        for entry in read_dir(directory).unwrap().filter_map(|e| e.ok()) {
            let entry_path: PathBuf = entry.path();
            println!("Entry: {:?}", &entry_path);

            if std::path::Path::is_file(&entry_path) {
                let lower_ext: String = entry_path
                    .extension()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_ascii_lowercase();

                let ext: &str = lower_ext.as_str();

                let option_file_type: Option<FILETYPE> = match ext {
                    "doc" | "docx" | "pp" | "pdf" | "csv" => Some(FILETYPE::DOCUMENT),
                    "js" | "css" | "scss" | "twig" => Some(FILETYPE::CODE),
                    "jpg" | "jpeg" | "png" => Some(FILETYPE::IMAGE),
                    "zip" | "rar" => Some(FILETYPE::FOLDERS),
                    "mp4" | "mov" => Some(FILETYPE::VIDEO),
                    "xd" | "ai" => Some(FILETYPE::DESIGN),
                    "mp3" => Some(FILETYPE::AUDIO),
                    _ => Some(FILETYPE::MISC),
                };

                if let Some(file_type) = option_file_type {
                    let result: Option<PathBuf> = move_file(entry_path, file_type);
                    if result.is_none() {
                        continue;
                    }
                } else {
                    eprintln!("Error parsing file type");
                }
            } else {
                println!("DIRECTORY FOUND. Running recursive mode.\n");

                let mut entries = std::fs::read_dir(&entry_path).unwrap();
                let is_empty: bool = entries.next().is_none();

                if is_empty {
                    std::fs::remove_dir(&entry_path).expect("Failed to remove empty directory.");
                } else {
                    let destination: PathBuf = PathBuf::from(config.folders_location.clone());
                    move_directory(entry_path, destination).expect("Failed to move directory");
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(3));
        println!("Clean Complete. Exiting...\n");
    }

    pub fn watch() {
        println!("\nWatching...");

        // TODO -> Run in a seperate thread and then detach it

        // loop {
        //     println!("\nWatch interval...");
        //     std::thread::sleep(std::time::Duration::from_secs(3));
        // }
    }

    pub fn purge() {
        // Run through archive and PURGE
    }

    pub fn kill() {
        // Kill any existing instaces of Sentry

        println!("\nStopping Sentry...\n");

        let mut system: System = System::new_all();
        system.refresh_all();

        let mut sentry_instances: Vec<(&Pid, &Process)> = Vec::new();
        let current_sentry: u32 = std::process::id();

        for (pid, process) in system.processes() {
            if process.name().eq_ignore_ascii_case("sentry_lite") {
                sentry_instances.push((pid, process));
            }
        }

        for (pid, process) in sentry_instances {
            if !pid.as_u32().eq(&current_sentry) {
                process.kill();
            }
        }

        println!("All Sentry instances stopped. Exiting...\n");
        std::process::exit(0);
    }
}
