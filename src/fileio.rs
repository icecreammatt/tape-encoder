use crate::metadata::*;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn write_metadata(data: OutputMetadata) {
    let directory = data.title.replace("_", "-").replace(" ", "-").replace(".", "-");

    if !fs::metadata(&directory).is_ok() {
        fs::create_dir(&directory).unwrap();
    }

    let json = serde_json::to_string(&data).unwrap();
    let outpath = format!("{}/{}.json", &directory, data.title);
    let mut file = File::create(outpath).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
