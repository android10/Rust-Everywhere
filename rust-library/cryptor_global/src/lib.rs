static CARGO_CONFIG_DIR_NAME: &str = ".cargo";
static CARGO_CONFIG_FILE_NAME: &str = "config";

pub mod io {
    use std::fs;
    use std::path::PathBuf;
    use std::fs::File;
    use std::io::Result;

    pub fn create_cargo_config_file(dir_path: &PathBuf) -> File {
        let config_dir = dir_path.join(super::CARGO_CONFIG_DIR_NAME);
        let config_file = config_dir.join(super::CARGO_CONFIG_FILE_NAME);

        match fs::create_dir(&config_dir) {
            Err(why) => println!("Couldn't create Cargo Configuration Directory: {}", why),
            Ok(_) => println!("Successfully created Cargo Configuration Directory."),
        }

        create_config_file(&config_file).expect("Could not create cargo configuration file")
    }

    fn create_config_file(file_path: &PathBuf) -> Result<File> {
        File::create(&file_path)
    }
}

#[cfg(test)]
mod tests {
    use super::io::*;

    mod io_tests {
        #[test]
        fn it_works() {
            assert_eq!(2 + 2, 4);
        }
    }
}