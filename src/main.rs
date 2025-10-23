use flexi_logger::{Duplicate, FileSpec};

mod audio;
mod database;
mod gui;
use flexi_logger::{Logger, WriteMode};

#[tokio::main]
pub async fn main() {
    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    std::fs::create_dir_all(".log").expect("Could not create .log directory");
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(FileSpec::default().directory(".log").basename("application").suffix("log"))
        // .duplicate_to_stdout(Duplicate::Info)
        .write_mode(WriteMode::BufferAndFlush) // or .Direct
        .start()
        .unwrap();
    // Logger::with_S
    log::info!("Logger initialized and writing to .log/application.log");

    // let audio_files = audio::scan_audio().expect("TODO: panic message");
    // if audio_files.is_empty() {
    //     panic!("nothing found to start")
    // }
    // log::info!("total music found:{}", audio_files.len());
    // database::create_tables().await;
    // database::insert_audio_details(&audio_files).await;
    
    let _ = gui::init().await;
}
