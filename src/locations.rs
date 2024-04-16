pub mod locations {
    use std::path::PathBuf;

    pub fn get_root_dir() -> PathBuf {
        return dirs::document_dir().unwrap();
    }

    pub fn get_config_dir() -> PathBuf {
        let mut config_root: PathBuf = dirs::config_dir().unwrap();
        config_root.push("Sentry_Lite");
        return config_root;
    }

    pub fn get_config_file_path() -> PathBuf {
        let mut config_file_path: PathBuf = get_config_dir();
        config_file_path.push("config.json");
        return config_file_path;
    }
}
