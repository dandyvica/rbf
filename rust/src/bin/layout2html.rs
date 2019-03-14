// Converts the XML layout file into an HTML file
use std::env;
use std::fs::write;

#[macro_use]
extern crate rbf;
use rbf::layout::Layout;
use rbf::record::AsciiMode;
use rbf::exportable::Exportable;

#[allow(unused_variables)]
fn main() {
    // get arguments
    let args: Vec<String> = env::args().collect();

    // get arguments
    if args.len() != 5 || !args.contains(&"-i".to_string()) || !args.contains(&"-o".to_string()) {
        println!("Usage: {} -i layout_file -o html_file", args[0]);
        std::process::exit(1);
    }

    // load layout (suppose only ascii data)
    let layout = error_check!(Layout::<AsciiMode>::new(&args[2]));

    // create output file
    write(&args[4], layout.to_html()).expect("Unable to write file");
}
