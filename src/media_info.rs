use crate::metadata::*;
use regex::Regex;
use std::process::Command;

pub fn get_media_info(input: &str) -> OutputMetadata {
    // TODO: Fix this so it works with . and - for date
    let re = Regex::new(r"(\d{4}-\d{2}-\d{2})?-(.*).(mp4)").unwrap();

    let captures = re.captures(input).expect("expect YYYY-MM-DD-File-Name.mp4");

    // println!("captures len {}", captures.len());
    // for cap in 0..captures.len() {
    // println!("cap: {}", cap);
    // println!("{}", format!("{}", captures.get(cap).unwrap().as_str()));
    // }

    // TODO: Make the date optional
    // let mut date = "";
    // if captures.len() < 4 {
    // }

    let date = captures.get(1).unwrap().as_str();
    let extracted_filename = captures.get(2).unwrap().as_str();
    let extension = captures.get(3).unwrap().as_str();

    let media_info = Command::new("mediainfo")
        .arg("--output=JSON")
        .arg(format!("{}", input))
        .output()
        .expect("mediainfo error parsing file");

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

    return metadata;
}
