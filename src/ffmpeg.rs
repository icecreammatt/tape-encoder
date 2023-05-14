use std::{fs, process::Command};

fn run_command(command: &str) {
    let cmd = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Error");

    println!("status: {}", cmd.status);

    println!("stdout: {}", String::from_utf8_lossy(&cmd.stdout));

    println!("stderr: {}", String::from_utf8_lossy(&cmd.stderr));

    let result = String::from_utf8_lossy(&cmd.stdout);
    let result = result.to_string();
    let result = result.as_str();

    println!("{}", result);
}

// fn create_preview_gif() {}

pub fn create_preview_image(input: String, output: String) {
    let path = format!("{}/lg", output);
    if !fs::metadata(&path).is_ok() {
        fs::create_dir_all(&path).unwrap();
    }

    let command = format!(
        "ffmpeg -i {} -vf scale=iw*sar:ih,setsar=1 -ss 00:00:05 -t 1 -vframes 1 {}/{}.jpg",
        input, path, output
    );

    run_command(&command);
}

pub fn create_thumbnails(input: String, output: String) {
    println!("\ncreate_thumbnails");

    let thumbs_path = format!("{}/thumbs", output);

    println!("Path: {}", thumbs_path);
    if !fs::metadata(&thumbs_path).is_ok() {
        fs::create_dir_all(&thumbs_path).unwrap();
    }

    let command = format!(
        "ffmpeg -i {} -vf \"fps=1/4,scale=320:-1\" {}/img%03d.jpg",
        input, thumbs_path
    );

    run_command(&command);
}

pub fn create_hls_encoding(input: String, output: String) {
    println!("\npub create_hls_encoding");

    if !fs::metadata(&output).is_ok() {
        fs::create_dir_all(&output).unwrap();
    }

    // TODO: Sub add paths
    // let paths = ["hls", "dash"];
    // for path in paths {
    //     let p = format!("{}/{}", path, output);
    //     if !fs::metadata(&p).is_ok() {
    //         fs::create_dir_all(&p).unwrap();
    //     }
    // }

    let command = format!("ffmpeg -stream_loop 0  -i {} \
        -map 0 -map 0 -map 0 -c:a aac -c:v h264_videotoolbox -allow_sw 1 \
        -b:v:0 800k -s:v:0 1280x720 -profile:v:0 main \
        -b:v:1 500k -s:v:1 640x340  -profile:v:1 main \
        -b:v:2 300k -s:v:2 320x170  -profile:v:2 baseline \
        -bf 1 \
        -keyint_min 120 -g 120 -sc_threshold 0 -b_strategy 0 -ar:a:1 22050 -use_timeline 1 -use_template 1 \
        -window_size 0 -adaptation_sets \"id=0,streams=v id=1,streams=a\" -hls_playlist 1 -seg_duration 4 \
        -streaming 0 -f dash \
        -hls_segment_filename \
         -hls_playlist_type {}/vod \
         {}/720p_%03d.m3u8 {}/720p.m3u8", input, output, output, output);
    run_command(&command);
}