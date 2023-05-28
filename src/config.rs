use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::path::Path;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    ffmpeg_command: String,
}

pub fn generate_config(path: PathBuf) -> Config {
    let config = Config {
        ffmpeg_command: String::from("test commnad with --flags testing"),
    };

    let file = File::create(path).expect("Error creating config file for writing");
    let _result = serde_yaml::to_writer(file, &config);
    config
}

pub fn is_directory_created(path: &PathBuf) -> bool {
    if fs::metadata(path).is_err() {
        return false;
    }
    true
}

fn get_config_path_from_home_dir(path: &str) -> PathBuf {
    let mut directory = PathBuf::new();
    directory.push(dirs::home_dir().unwrap());
    directory.push(path);
    directory
}

fn get_config_path(directory: &PathBuf, file: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(directory);
    path.push(file);
    path
}

pub fn load_config() -> Config {
    let directory = get_config_path_from_home_dir(".config/tape-encoder/");
    let config_file = get_config_path(&directory, "config.yaml");

    if !is_directory_created(&directory) {
        if !Path::new(&directory).is_dir() {
            match fs::create_dir(directory) {
                Ok(()) => println!("Config does not exist initilizing with default config"),
                Err(e) => println!("Problem creating config path: {}", e),
            }
        }
        generate_config(config_file)
    } else {
        let file = File::open(config_file).expect("unable to open file");
        let config: Config = serde_yaml::from_reader(file).expect("unable to extract");
        config
    }
}
