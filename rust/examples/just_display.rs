// Simple exemple of rbf usage. Just read the whole file
use std::env;

#[macro_use]
extern crate rbf;
use rbf::layout::Layout;
use rbf::reader::Reader;
use rbf::record::AsciiMode;

fn main() {
    // get arguments
    let args: Vec<String> = env::args().collect();

    // get arguments
    if args.len() == 1 {
        println!("Usage: {} layout_file data_file", args[0]);
        std::process::exit(1);
    }

    // load layout (suppose only ascii data)
    let layout = error_check!(Layout::<AsciiMode>::new(&args[1]));

    // create reader
    let mut reader = Reader::new(&args[2], layout);

    // loop through records
    while let Some((_stats, rec)) = reader.next() {
        println!("{}", rec);
    }
}
