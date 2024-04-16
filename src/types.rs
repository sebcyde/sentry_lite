pub mod types {
    use serde::{Deserialize, Serialize};

    pub enum FILETYPE {
        DOCUMENT,
        ARCHIVE,
        FOLDERS,
        DESIGN,
        VIDEO,
        AUDIO,
        IMAGE,
        CODE,
        MISC,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct UserConfig {
        pub documents_location: String,
        pub archive_location: String,
        pub folders_location: String,
        pub design_location: String,
        pub video_location: String,
        pub audio_location: String,
        pub image_location: String,
        pub code_location: String,
        pub misc_location: String,
    }

    impl UserConfig {
        pub fn any_field_empty(&self) -> bool {
            self.documents_location.is_empty()
                || self.archive_location.is_empty()
                || self.folders_location.is_empty()
                || self.design_location.is_empty()
                || self.video_location.is_empty()
                || self.audio_location.is_empty()
                || self.image_location.is_empty()
                || self.code_location.is_empty()
                || self.misc_location.is_empty()
        }
    }

    impl Default for UserConfig {
        fn default() -> Self {
            UserConfig {
                folders_location: String::new(),
                archive_location: String::new(),
                audio_location: String::new(),
                code_location: String::new(),
                design_location: String::new(),
                documents_location: String::new(),
                image_location: String::new(),
                misc_location: String::new(),
                video_location: String::new(),
            }
        }
    }
}
