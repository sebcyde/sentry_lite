pub mod actions {
    use std::{ffi::OsStr, fs::read_dir, path::PathBuf};

    use sysinfo::{Pid, Process, System};

    use crate::{
        config::config::get_config_data,
        types::types::{UserConfig, FILETYPE},
        utils::utils::{move_directory, move_file, remove_dir_if_empty},
    };

    pub fn clean(directory: PathBuf) {
        println!("\nCleaning...");
        let config: UserConfig = get_config_data();

        for entry in read_dir(directory).unwrap().filter_map(|e| e.ok()) {
            // std::thread::sleep(std::time::Duration::from_secs(2));

            let entry_path: PathBuf = entry.path();
            println!("Entry: {:?}", &entry_path);

            if std::path::Path::is_file(&entry_path) {
                let raw_lower_ext: Option<&OsStr> = entry_path.extension();
                if raw_lower_ext.is_none() {
                    _ = move_file(entry_path, FILETYPE::MISC);
                    continue;
                }

                let lower_ext: String = raw_lower_ext
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_ascii_lowercase();

                let ext: &str = lower_ext.as_str();

                let option_file_type: Option<FILETYPE> = match ext {
                    "doc" | "docx" | "pp" | "pdf" | "csv" => Some(FILETYPE::DOCUMENT),
                    "jpg" | "jpeg" | "png" | "svg" => Some(FILETYPE::IMAGE),
                    "js" | "css" | "scss" | "twig" => Some(FILETYPE::CODE),
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
                let remove_res: Option<&PathBuf> = remove_dir_if_empty(&entry_path);

                println!("remove_res = {:?}", remove_res);

                if remove_res.is_some() {
                    let mut destination: PathBuf = PathBuf::from(config.folders_location.clone());

                    if destination.to_str().unwrap().eq_ignore_ascii_case("") {
                        let mut misc_dir_raw: PathBuf = dirs::document_dir().unwrap();
                        misc_dir_raw.push("Sentry");
                        misc_dir_raw.push("Misc_Dirs");

                        std::fs::create_dir_all(&misc_dir_raw)
                            .expect("Failed to create misc directory.");

                        destination = misc_dir_raw;
                        println!("\n\nDIR SKIPPED -- MOVED TO MISC DIR\n\n");
                    }

                    println!("destination => {:?}", &destination);

                    move_directory(entry_path, destination).expect("Failed to move directory");
                } else {
                    continue;
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(3));
        println!("\nClean Complete. Exiting...\n");
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
