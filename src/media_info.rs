use crate::metadata::*;
use regex::Regex;
use std::fs;
use std::process::Command;

fn get_file_parts(file: &str) -> (&str, &str, &str) {
    let re = Regex::new(r"(\d{4}(?:-|\.)\d{2}(?:-|\.)\d{2})?(?:-|\.)*(.+).(mp4|mov)")
        .expect("File format: YYYY-MM-DD-File.mp4");
    let captures = re.captures(file).expect("expect YYYY-MM-DD-File-Name.mp4");

    let date = captures.get(1).map_or("No Date", |x| x.as_str());
    let extracted_filename = captures.get(2).unwrap().as_str();
    let extension = captures.get(3).unwrap().as_str();

    (date, extracted_filename, extension)
}

pub fn get_media_info(input: &str) -> OutputMetadata {
    if fs::metadata(input).is_err() {
        println!("{} is not found.", &input);
        std::process::exit(1);
    }

    let (date, extracted_filename, extension) = get_file_parts(input);

    let media_info = Command::new("mediainfo")
        .arg("--output=JSON")
        .arg(input)
        .output()
        .expect("mediainfo error parsing file, make sure mediainfo is installed");

    // if output.status == std::process::Output::Err(output.status) {
    //     panic!("Error reading file info");
    // }

    let result = String::from_utf8_lossy(&media_info.stdout);
    let result = result.to_string();
    let result = result.as_str();

    let metadata: MediaInfo = serde_json::from_str(result).unwrap();
    let media = metadata.media;

    let mut metadata = OutputMetadata {
        date: date.to_string(),
        title: extracted_filename.to_string(),
        file_name: media.name,
        frame_count: 0,
        frame_rate: 0.0,
        duration_seconds: 0.0,
        duration_human: "".to_string(),
        extension: extension.to_string(),
        width: 0,
        height: 0,
    };

    for track in media.track {
        // println!("{:#?}", track);
        if track.data_type == "Video" {
            metadata.frame_count = track.frame_count.parse().unwrap();
            let duration = DurationSeconds {
                duration_seconds: track.duration.parse().unwrap(),
            };
            metadata.duration_seconds = duration.duration_seconds;
            metadata.duration_human = duration.duration_human();
            metadata.frame_rate = track.frame_rate.parse().unwrap();

            match track.width {
                Some(width) => metadata.width = width.parse().unwrap(),
                None => {}
            }
            match track.height {
                Some(height) => metadata.height = height.parse().unwrap(),
                None => {}
            }
        }
    }

    metadata
}

#[test]
fn file_name_with_date() {
    let input = "2022-02-02-filename.mp4";
    let (date, extracted_filename, extension) = get_file_parts(input);

    assert_eq!(date, "2022-02-02");
    assert_eq!(extracted_filename, "filename");
    assert_eq!(extension, "mp4");
}

#[test]
fn file_name_without_date() {
    let input = "filename.mp4";
    let (date, extracted_filename, extension) = get_file_parts(input);

    assert_eq!(date, "No Date");
    assert_eq!(extracted_filename, "filename");
    assert_eq!(extension, "mp4");
}

#[test]
#[should_panic]
fn wrong_extension() {
    let input = "filename.ext";
    let (_date, _extracted_filename, _extension) = get_file_parts(input);
}

#[test]
#[should_panic]
fn no_extension() {
    let input = "filename";
    let (_date, _extracted_filename, _extension) = get_file_parts(input);
}
