#![feature(iter_intersperse)]

mod app;
mod config;
mod encode_term;
mod event;
mod keymap;
mod proc;
mod state;
mod theme;
mod ui_keymap;
mod ui_procs;
mod ui_term;

use flexi_logger::FileSpec;

use crate::app::App;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  let _logger = flexi_logger::Logger::try_with_str("info")
    .unwrap()
    .log_to_file(FileSpec::default().suppress_timestamp())
    .use_utc()
    .start()
    .unwrap();

  let app = App::new();
  match app.run().await {
    Ok(()) => Ok(()),
    Err(err) => {
      eprintln!("Error: {}", err);
      Ok(())
    }
  }
}
