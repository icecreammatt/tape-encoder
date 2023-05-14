use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DurationSeconds {
    pub duration_seconds: f32,
}

impl DurationSeconds {
    pub fn duration_human(&self) -> String {
        let hours = self.duration_seconds / 3600.0;
        let minutes = (self.duration_seconds % 3600.0) / 60.0;
        let seconds = (self.duration_seconds % 3600.0) % 60.0;

        if hours.floor() < 1.0 {
            return format!("{:02}:{:02}", minutes.floor(), seconds.floor());
        }

        format!(
            "{:02}:{:02}:{:02}",
            hours.floor(),
            minutes.floor(),
            seconds.floor()
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OutputMetadata {
    pub title: String,
    pub file_name: String,
    pub extension: String,
    pub date: String,
    pub duration_human: String,
    pub duration_seconds: f32,
    pub frame_count: u32,
    pub frame_rate: f32,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatingLibrary {
    name: String,
    version: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackGeneral {
    #[serde(rename = "@type")]
    pub data_type: String,
    #[serde(rename = "FrameCount")]
    pub frame_count: String,
    #[serde(rename = "FrameRate")]
    pub frame_rate: String,
    #[serde(rename = "FileExtension")]
    pub file_extension: Option<String>,
    #[serde(rename = "Duration")]
    pub duration: String,
    #[serde(rename = "Width")]
    pub width: Option<String>,
    #[serde(rename = "Height")]
    pub height: Option<String>,
    #[serde(rename = "Stored_Width")]
    pub stored_width: Option<String>,
    #[serde(rename = "Stored_Height")]
    pub stored_height: Option<String>,
    #[serde(rename = "DisplayAspectRatio")]
    pub display_aspect_ratio: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Media {
    #[serde(rename = "@ref")]
    pub name: String,
    pub track: Vec<TrackGeneral>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaInfo {
    #[serde(rename = "creatingLibrary")]
    pub creating_library: CreatingLibrary,
    pub media: Media,
}

#[test]
fn test_duration_conversion_pad_with_zero_one_hour() {
    let input = DurationSeconds {
        duration_seconds: 7200.0,
    };

    assert_eq!(input.duration_human(), "02:00:00");
}

#[test]
fn sample_from_test_file() {
    let input = DurationSeconds {
        duration_seconds: 149.316,
    };

    assert_eq!(input.duration_human(), "02:29");
}

#[test]
fn test_duration_conversion_pad_with_zero_no_hour() {
    let input = DurationSeconds {
        duration_seconds: 360.0,
    };

    assert_eq!(input.duration_human(), "06:00");
}
