pub mod actions {
    use std::{fs::read_dir, path::PathBuf};

    use sysinfo::{Pid, Process, System};

    use crate::utils::utils::{move_file, FILETYPE};

    pub fn clean(directory: PathBuf) {
        println!("\nCleaning...");
        // std::thread::sleep(std::time::Duration::from_secs(3));
        // println!("Clean Complete. Exiting...\n");

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
                    "jpg" | "jpeg" | "png" => Some(FILETYPE::IMAGE),
                    "mp4" | "mov" => Some(FILETYPE::VIDEO),
                    "mp3" => Some(FILETYPE::AUDIO),
                    "xd" | "ai" => Some(FILETYPE::DESIGN),
                    "doc" | "docx" | "pdf" | "csv" => Some(FILETYPE::DOCUMENT),
                    "zip" | "rar" => Some(FILETYPE::ARCHIVE),
                    "js" | "css" | "scss" | "twig" => Some(FILETYPE::CODE),
                    _ => None,
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
                clean(entry_path);
            }
        }
    }

    pub fn watch() {
        println!("\nWatching...");

        // Run in a seperate thread and then detach it

        loop {
            println!("\nWatch interval...");
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    }

    pub fn purge() {
        // Run through old stuff and PURGE
    }

    pub fn kill() {
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
