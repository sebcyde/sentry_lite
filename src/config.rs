pub mod config {
    use std::fs::{read_to_string, File};
    use std::io::Write;
    use std::path::PathBuf;

    use crate::locations::locations::{get_config_dir, get_config_file_path};
    use crate::types::types::UserConfig;
    use crate::utils::utils::exit_prompt;

    fn get_default_user_config() -> UserConfig {
        let empty_user_config: UserConfig = UserConfig::default();
        println!("{:?}", empty_user_config);
        return empty_user_config;
    }

    fn get_user_config() -> UserConfig {
        let config_value: &str = &read_to_string(get_config_file_path()).unwrap();
        let config: UserConfig = serde_json::from_str(config_value).unwrap();
        return config;
    }

    fn create_config_dir_and_files() {
        let config_dir_path: PathBuf = get_config_dir();

        let dir_res: Result<(), std::io::Error> = std::fs::create_dir_all(&config_dir_path);

        if dir_res.is_err() {
            eprintln!("Error creating config directory.");
        }

        create_default_config_files();
    }

    fn create_default_config_files() {
        let default_config: UserConfig = get_default_user_config();
        let config_file_path: PathBuf = get_config_file_path();

        let mut config_file: File = File::create(&config_file_path).unwrap();
        let json_data: String = serde_json::to_string(&default_config).unwrap();
        _ = config_file.write_all(json_data.as_bytes());

        exit_prompt(&format!("Config created succesfully. Please fill in necessary paths and then run again.\nConfig location: {:?}", &config_file_path));
    }

    pub fn check_config() -> bool {
        println!("\nChecking config...");

        let config_file_path: PathBuf = get_config_file_path();
        let config_dir_path: PathBuf = get_config_dir();

        if std::path::Path::exists(&config_dir_path) {
            if std::path::Path::exists(&config_file_path) {
                // Dir and config Exists

                let config: UserConfig = get_user_config();

                if config.any_field_empty() {
                    println!("Config directories not set. Using default.");
                    println!("{}", format!("Custom destination paths can be set in the config.json.\nConfig location: {:?}", &config_file_path));

                    return false;
                } else {
                    // Config loaded succesffully
                    return true;
                }
            } else {
                // Dir exists but no config
                create_default_config_files();
                return false;
            }
        } else {
            // Dir and config missing
            println!("Creating config. Please wait...");
            create_config_dir_and_files();
            return false;
        }
    }

    pub fn get_config_data() -> UserConfig {
        let config_file_path: PathBuf = get_config_file_path();
        let config_value: &str = &read_to_string(config_file_path).unwrap();
        let config: UserConfig = serde_json::from_str(config_value).unwrap();
        return config;
    }
}
