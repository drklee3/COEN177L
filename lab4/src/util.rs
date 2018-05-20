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
pub fn setup_logger(verbosity: u64) -> Result<()> {
  let colors = ColoredLevelConfig::new()
    .info(Color::BrightGreen)
    .debug(Color::BrightCyan);
  
  let mut base_config = fern::Dispatch::new();

  base_config = match verbosity {
    0 => base_config
      .level(log::LevelFilter::Info),
    1 => base_config
      .level(log::LevelFilter::Debug),
    _2_or_more => base_config // nothing really uses trace tho
      .level(log::LevelFilter::Trace),
  };

  base_config
    .format(move |out, message, record| {
      out.finish(format_args!(
        "{}[{}] {}",
        chrono::Local::now().format("[%y%m%d %H:%M:%S]"),
        colors.color(record.level()),
        message
      ))
    })
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
