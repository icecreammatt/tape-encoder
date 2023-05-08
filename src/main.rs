use clap::{App, Arg};
use std::process;

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
                .help("Sets the input file to use")
                .takes_value(true),
        )
        .get_matches();

    let input = matches.value_of("input").unwrap_or("help");
    // println!("Input file: {}", input);

    if input == "help" {
        println!("Usage: <TODO>");

        process::exit(1);
    }

    let out = media_info::get_media_info(&input);
    println!("{:#?}", out);
    fileio::write_metadata(out);

    /*
    TODO:
    [x] Take filename as argument to process
    [x] Read metadata from filename
    [x] Read metadata with mediainfo from video file
    [x] Make directory for metadata
    [x] Store Metadata in JSON

    [ ] FFMPEG to generate media
        [ ] thumbnails
        [ ] gif
        [ ] preview image
        [ ] HLS
        [ ] DASH
        [ ] Metadata
    [ ] Upload metadata to database

    [ ] Create queue watcher to start running jobs (watches queueu every 5 seconds)
        - On new item run generator process
    */
}
