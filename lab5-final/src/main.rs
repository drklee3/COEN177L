extern crate csv;
extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::error::Error;
use std::env;
use std::process;
use std::io::Read;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Requires input and output file");
        process::exit(1);
    }
    let mut file = File::open(&args[1]).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let data = parse_bench(&contents);
    let _ = save_file(data, &args[2]);
    println!("done");
}

#[derive(Debug, Serialize)]
struct Data {
    thread_count: usize,
    lock_percentage: u64,
    time: u64,
}

fn parse_bench(text: &str) -> Vec<Data> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"test (\w*)_(\w*)_(\w*) .* ([\d,]*) ns").unwrap();
    }

    let mut data = Vec::new();

    for cap in RE.captures_iter(text) {
        let thread_count = cap[2]
            .replace("t", "")
            .parse::<usize>()
            .unwrap();

        let lock_percentage = cap[3]
            .replace("p", "")
            .parse::<u64>()
            .unwrap();

        let time = cap[4]
            .replace(",", "")
            .parse::<u64>()
            .unwrap();

        data.push(Data {
            thread_count,
            lock_percentage,
            time,
        });
    }

    data
}

fn save_file(data: Vec<Data>, output: &str) -> Result<(), Box<Error>> {
    let mut wtr = csv::Writer::from_path(output)?;
    
    for value in data {
        wtr.serialize(value)?;
    }
    wtr.flush()?;

    Ok(())
}
