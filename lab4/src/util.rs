use fern;
use fern::colors::{
  Color,
  ColoredLevelConfig,
};
use log;
use chrono;
use std;
use error::{Error, Result};

/// Sets up the logger
pub fn setup_logger() -> Result<()> {
  let colors = ColoredLevelConfig::new()
    .info(Color::BrightGreen)
    .debug(Color::BrightCyan);

  fern::Dispatch::new()
    .format(move |out, message, record| {
      out.finish(format_args!(
        "{}[{}][{}] {}",
        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
        record.target(),
        colors.color(record.level()),
        message
      ))
    })
    .level(log::LevelFilter::Debug)
    .chain(std::io::stderr())
    .chain(fern::log_file("output.log")?)
    .apply()?;

  Ok(())
}

/// Parses command line arguments to get a valid page table size
pub fn parse_args() -> Result<usize> {
  let args: Vec<String> = std::env::args().collect();

  if args.len() != 2 {
    return Err(Error::from("Missing argument for table size."));
  }

  args[1]
    .parse::<usize>()
    .map_err(From::from)
}

/// Checks if a page request is in the page table
pub fn is_in_memory(request: u64, page_table: &Vec<u64>) -> bool {
  page_table.contains(&request)
}
