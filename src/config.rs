use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{self},
};

const DIRECTORY: &str = "~/.config/tape-encoder/";
// const CONFIG_PATH: &str = format!("{}{}", DIRECTORY, "config.yaml").as_str();

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    ffmpeg_command: String,
}

// pub fn generate_config() -> io::Result<(&Config)> {
pub fn generate_config() -> Config {
    // fs::create_dir(&DIRECTORY)?;

    let config = Config {
        ffmpeg_command: String::from("test commnad with --flags testing"),
    };

    /*
    let yaml = serde_yaml::to_string(&config);
    let mut file = File::create(CONFIG_PATH)?;
    if Some(file) {}
    let result = file.write_all(yaml.as_bytes())?;
    Ok((result))
    */
    config
}

pub fn check_for_config(path: &str) -> bool {
    if fs::metadata(&path).is_err() {
        return false;
    }
    return true;
}

pub fn load_config() -> Config {
    // if !check_for_config(CONFIG_PATH) {
    // generate_config()
    // } else {
    generate_config()
    // }

    // let file = File::open(CONFIG_PATH)?;
    // let config: Config = serde_yaml::from_reader(file)?;

    // config
}
