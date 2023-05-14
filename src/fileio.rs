use crate::metadata::*;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn file_to_hyphen(name: &String) -> String {
    return String::from(name.replace("_", "-").replace(" ", "-").replace(".", "-"));
}

pub fn write_metadata(data: &OutputMetadata) {
    let directory = file_to_hyphen(&data.title);

    if !fs::metadata(&directory).is_ok() {
        fs::create_dir(&directory).unwrap();
    }

    let json = serde_json::to_string(&data).unwrap();
    let outpath = format!("{}/{}.json", &directory, data.title);
    let mut file = File::create(outpath).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

#[test]
fn file_to_hyphen_test() {
    let result = file_to_hyphen(&"2022.04.06.name with space.mp4".to_string());
    assert_eq!(result, "2022-04-06-name-with-space-mp4");
}
