pub mod actions {
    use std::{ffi::OsStr, fs::read_dir, path::PathBuf, process::Output};

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
            std::thread::sleep(std::time::Duration::from_millis(100));

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
        std::thread::spawn(move || loop {
            clean(dirs::download_dir().unwrap());
            std::thread::sleep(std::time::Duration::from_secs(3));
        });
    }

    pub fn kill() {
        // Kill any existing instaces of Sentry - Kill any watchers

        println!("\nStopping Sentry...\n");

        let current_sentry: u32 = std::process::id();
        println!("Current Sentry PID: {:?}", current_sentry);

        // Filter out Sentry instances
        let output: Output = std::process::Command::new("ps")
            .arg("-ax")
            .output()
            .expect("Failed to execute command");

        let output_string = String::from_utf8_lossy(&output.stdout);

        // Iterate over lines in the output
        for line in output_string.lines() {
            // Check if the line contains "sentry_lite"
            if line.contains("sentry_lite") {
                let pid: &str = line.split_whitespace().next().unwrap();

                if let Ok(pid_u32) = pid.parse::<u32>() {
                    if pid_u32 != current_sentry {
                        let _ = std::process::Command::new("kill").arg(pid).status();
                        println!("Killed Sentry instance with PID {}", pid);
                    }
                }
            }
        }

        println!("All Sentry instances stopped. Exiting...\n");
        std::process::exit(0);
    }

    pub fn purge() {
        // Run through archive and PURGE
    }
}
