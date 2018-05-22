use chrono;
use csv::Writer;
use error::{Error, Result};
use fern::{
  self,
  colors::{
    Color,
    ColoredLevelConfig,
  }
};
use log;
use std;

/// Sets up the logger
pub fn setup_logger(verbosity: u64) -> Result<()> {
  let colors = ColoredLevelConfig::new()
    .info(Color::BrightGreen)
    .debug(Color::BrightCyan)
    .trace(Color::BrightMagenta);
  
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

pub fn validate_table_size(size: String) -> std::result::Result<(), String> {
  if let Ok(parsed) = size.parse::<usize>() {
    if parsed <= 0 {
      // don't think we can get negative numbers so this is
      // mainly just a check for 0
      return Err("Please give a number over 0".into());
    }
  } else {
    return Err("Please give a number".into());
  }

  Ok(())
}

pub fn save_result(output: &str, algorithm: &str, hit_rates: Vec<(usize, f64)>)
  -> Result<()> {
  // format output with algorithm name
  let output = format!("{}.{}.csv", output.replace(".csv", ""), algorithm);

  // create new writer
  let mut wtr = Writer::from_path(&output)?;
  // write header
  wtr.write_record(&["table_size", algorithm])?;
  // write each hit rate
  for record in hit_rates {
    wtr.write_record(&[record.0.to_string(), record.1.to_string()])?;
  }
  wtr.flush()?;
  
  info!("Saved hit rate data to {}", &output);
  Ok(())
}