use std::{fs, io, process::Command};

fn run_command(command: &str) -> io::Result<()> {
    let cmd = Command::new("sh").arg("-c").arg(command).output()?;

    println!("status: {}", cmd.status);

    println!("stdout: {}", String::from_utf8_lossy(&cmd.stdout));

    println!("stderr: {}", String::from_utf8_lossy(&cmd.stderr));

    let result = String::from_utf8_lossy(&cmd.stdout);
    let result = result.to_string();
    let result = result.as_str();

    println!("{}", result);
    Ok(())
}

pub fn create_preview_gif(input: &str, subpath: &str, output: &str) -> io::Result<()> {
    let path = format!("{}/{}", output, subpath);
    if fs::metadata(&path).is_err() {
        fs::create_dir_all(&path)?;
    }

    let command = format!(
    "ffmpeg -ss 30 -t 3 -i {} -vf \"fps=10,scale=320:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse\" -loop 0 {}/{}.gif", input, path, output
    );

    run_command(&command)?;
    Ok(())
}

pub fn create_preview_image(input: &str, subpath: &str, output: &str) -> io::Result<()> {
    let path = format!("{}/{}", output, subpath);
    if fs::metadata(&path).is_err() {
        fs::create_dir_all(&path)?;
    }

    let command = format!(
        "ffmpeg -i {} -vf scale=iw*sar:ih,setsar=1 -ss 00:00:05 -t 1 -vframes 1 {}/{}.jpg",
        input, path, output
    );

    run_command(&command)?;
    Ok(())
}

pub fn create_thumbnails(input: &str, subpath: &str, output: &str) -> io::Result<()> {
    println!("\ncreate_thumbnails");

    let thumbs_path = format!("{}/{}", output, subpath);

    println!("Path: {}", thumbs_path);
    if fs::metadata(&thumbs_path).is_err() {
        fs::create_dir_all(&thumbs_path)?;
    }

    let command = format!(
        "ffmpeg -i {} -vf \"fps=1/4,scale=320:-1\" {}/img%03d.jpg",
        input, thumbs_path
    );

    run_command(&command)?;
    Ok(())
}

pub fn create_hls_encoding(input: &str, subpath: &str, output: &str) -> io::Result<()> {
    println!("\npub create_hls_encoding");

    let hls_path = format!("{}/{}/hls", output, subpath);

    if fs::metadata(&hls_path).is_err() {
        fs::create_dir_all(&hls_path)?;
    }

    let dash_path = format!("{}/{}/dash", output, subpath);

    if fs::metadata(&dash_path).is_err() {
        fs::create_dir_all(&dash_path)?;
    }

    let command = format!("ffmpeg -stream_loop 0  -i {} \
        -map 0 -map 0 -map 0 -c:a aac -c:v h264_videotoolbox -allow_sw 1 \
        -b:v:0 800k -s:v:0 1280x720 -profile:v:0 main \
        -b:v:1 500k -s:v:1 640x340  -profile:v:1 main \
        -b:v:2 300k -s:v:2 320x170  -profile:v:2 baseline \
        -bf 1 \
        -keyint_min 120 -g 120 -sc_threshold 0 -b_strategy 0 -ar:a:1 22050 -use_timeline 1 -use_template 1 \
        -window_size 0 -adaptation_sets \"id=0,streams=v id=1,streams=a\" -hls_playlist 1 -seg_duration 4 \
        -streaming 0 -f dash \
        -hls_playlist_type {}/vod \
        {}/720p_%03d.m3u8 {}/720p_%03d.m3u8", input, hls_path, hls_path, dash_path);
    run_command(&command)?;
    Ok(())
}
// -hls_segment_filename \
