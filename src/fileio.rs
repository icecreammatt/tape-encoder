use crate::metadata::*;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn file_to_hyphen(name: &str) -> String {
    name.replace(['_', ' ', '.'], "-")
}

pub fn write_metadata(data: &OutputMetadata) {
    let directory = file_to_hyphen(&data.title);

    if fs::metadata(&directory).is_err() {
        fs::create_dir(&directory).unwrap();
    }

    let json = serde_json::to_string(&data).unwrap();
    let outpath = format!("{}/{}.json", &directory, data.title);
    let mut file = File::create(outpath).unwrap();
    file.write_all(json.as_bytes()).unwrap();
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

    let _result = write_metadata(&output_metadata);

    let file = File::open("temp/temp.json").expect("error opening file");
    let reader = std::io::BufReader::new(file);
    let metadata: OutputMetadata = serde_json::from_reader(reader).expect("unable to read json");

    assert_eq!(metadata.title, "temp")
}
