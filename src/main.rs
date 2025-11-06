use flexi_logger::{Duplicate, FileSpec};
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod audio;
mod database;

mod gui;
use crate::database::repository;
use flexi_logger::{Logger, WriteMode};
// use crate::audio;

#[tokio::main]
pub async fn main() {
    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    std::fs::create_dir_all(".log").expect("Could not create .log directory");
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory(".log")
                .basename("application")
                .suffix("log"),
        )
        //.duplicate_to_stdout(Duplicate::Info)
        .write_mode(WriteMode::BufferAndFlush) // or .Direct
        .start()
        .unwrap();
    // Logger::with_S
    log::info!("Logger initialized and writing to .log/application.log");
    //rayon init
    let num_cpus = num_cpus::get();
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus)
        .build_global()
        .ok();

    audio::load_audio_on_startup().await;
    let _ = gui::init().await;

    // lets play song here

    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let file =
        File::open("C:\\Users\\mechanic\\Downloads\\divide-ed-sheeran\\07 Happier.mp3").unwrap();
    let _sink = rodio::play(&stream_handle.mixer(), file).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(100));
}
