use crate::metadata::*;
use std::fs::File;
use std::io::Write;
use std::{fs, io};

pub fn file_to_hyphen(name: &str) -> String {
    name.replace(['_', ' ', '.'], "-")
}

pub fn write_metadata(data: &OutputMetadata) -> io::Result<()> {
    let directory = file_to_hyphen(&data.title);

    if fs::metadata(&directory).is_err() {
        fs::create_dir(&directory)?;
    }

    let json = serde_json::to_string(&data)?;
    let outpath = format!("{}/{}.json", &directory, data.title);
    let mut file = File::create(outpath)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

#[test]
fn file_to_hyphen_test() {
    let result = file_to_hyphen("2022.04.06.name with space.mp4");
    assert_eq!(result, "2022-04-06-name-with-space-mp4");
}

#[test]
fn write_metadata_test() {
    let output_metadata = OutputMetadata {
        title: "temp".to_string(),
        ..Default::default()
    };

    let result = write_metadata(&output_metadata);
    match result {
        Ok(()) => {}
        Err(e) => panic!("Failed to write metadata file: {}", e),
    }

    let file = File::open("temp/temp.json").expect("error opening file");
    let reader = std::io::BufReader::new(file);
    let metadata: OutputMetadata = serde_json::from_reader(reader).expect("unable to read json");

    assert_eq!(metadata.title, "temp")
}
