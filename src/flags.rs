pub enum Flags {
    Gif,
    Hls,
    _Metadata,
    PreviewImage,
    Thumbnails,
    Help,
}

// impl fmt::Display for Flags {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Flags::_Gif => write!(f, "gif"),
//             Flags::HLS => write!(f, "HLS"),
//             Flags::_Metadata => write!(f, "metadata"),
//             Flags::_PreviewImage => write!(f, "preview_image"),
//             Flags::Thumbnails => write!(f, "thumbs"),
//         }
//     }
// }

impl ToString for Flags {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl Flags {
    pub fn as_str(&self) -> &'static str {
        match self {
            Flags::Gif => "gif",
            Flags::Hls => "HLS",
            Flags::_Metadata => "metadata",
            Flags::PreviewImage => "preview_image",
            Flags::Thumbnails => "thumbs",
            Flags::Help => "help",
        }
    }
}
