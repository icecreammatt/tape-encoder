use crate::ffmpeg::{create_preview_gif, create_preview_image};
use clap::{App, Arg};
use std::process;

enum Flags {
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
    fn as_str(&self) -> &'static str {
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

use crate::{
    ffmpeg::{create_hls_encoding, create_thumbnails},
    fileio::file_to_hyphen,
};

mod ffmpeg;
mod fileio;
mod media_info;
mod metadata;

fn main() {
    let matches = App::new("tape-encoder")
        .version("1.0")
        .author("Matt Carrier")
        .about("Convert video to streamable pieces")
        .arg(
            Arg::with_name("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Sets the input file to use"),
        )
        .arg(
            Arg::with_name(Flags::Thumbnails.as_str())
                .short('t')
                .long(Flags::Thumbnails.as_str())
                .help("Generate thumbnails")
                .takes_value(false),
        )
        .arg(
            Arg::with_name(Flags::PreviewImage.as_str())
                .short('p')
                .long(Flags::PreviewImage.as_str())
                .help("Generate preview")
                .takes_value(false),
        )
        .arg(
            Arg::with_name(Flags::Hls.as_str())
                .short('h')
                .long(Flags::Hls.as_str())
                .help("Generate hls chunks")
                .takes_value(false),
        )
        .arg(
            Arg::with_name(Flags::Gif.as_str())
                .short('g')
                .long(Flags::Gif.as_str())
                .help("Generate gif")
                .takes_value(false),
        )
        .get_matches();

    let input = matches.value_of("input").unwrap_or(Flags::Help.as_str());
    if input == Flags::Help.as_str() {
        println!("Usage: <TODO>");
        process::exit(1);
    }

    let out = media_info::get_media_info(input);
    println!("{:#?}", &out);
    fileio::write_metadata(&out);

    let path = format!("./{}", file_to_hyphen(&out.title));

    let gen_thumbs = matches.is_present(Flags::Thumbnails.as_str());
    if gen_thumbs {
        create_thumbnails(&out.file_name, &path);
    }

    let gen_preview_image = matches.is_present(Flags::PreviewImage.as_str());
    if gen_preview_image {
        create_preview_image(&out.file_name, &path);
    }

    let gen_hls = matches.is_present(Flags::Hls.as_str());
    if gen_hls {
        create_hls_encoding(&out.file_name, &path);
    }

    let gen_gif = matches.is_present(Flags::Gif.as_str());
    if gen_gif {
        create_preview_gif(&out.file_name, &path);
    }

    /*
    TODO:
    [x] Take filename as argument to process
    [x] Read metadata from filename
    [x] Read metadata with mediainfo from video file
    [x] Make directory for metadata
    [x] Store Metadata in JSON

    [ ] FFMPEG to generate media
        [x] thumbnails
        [ ] gif
        [x] preview image
        [x] HLS
        [x] DASH
        [ ] Metadata
    [ ] Upload metadata to database

    [ ] Create queue watcher to start running jobs (watches queueu every 5 seconds)
        - On new item run generator process
    */
}
