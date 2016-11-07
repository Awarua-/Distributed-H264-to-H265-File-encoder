#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate clap;

use clap::App;
use std::process::Command;
use std::io::ErrorKind::NotFound;

struct Output {
    status: bool,
    stdout: String,
    stderr: String,
}

fn checkArgs(command: String, args: Vec<String>) -> Output {
    let output = Command::new(command)
                        .args(args.as_slice())
                        .output()
                        .expect("Failed to execute");
    Output {status: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string()}
}


fn main() {

    // log4rs::init_file("../log4rs.yaml", Default::default()).unwrap();

    // info!("starting up");
    let yaml = load_yaml!("../cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let ffmpeg = checkArgs("dir".to_string(), vec![]);


    // let output = Command::new("ffmpegf")
    // .output()
    // .expect("failed to execute proces");
    //
    println!("ffmpeg stderr: {}", ffmpeg.stderr);
    println!("ffmpeg stdout: {}", ffmpeg.stdout);
    println!("ffmpeg status: {}", ffmpeg.status);
    //
    // let ffprobe = Command::new("ffprobe")
    // .output()
    // .expect("failed to execute proces");
    //
    // println!("ffprobe version: {}", ffprobe.status);
}
