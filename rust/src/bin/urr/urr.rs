// Simple exemple of rbf usage. Just read the whole file
use std::env;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

extern crate regex;
use regex::Regex;

#[macro_use]
extern crate rbf;
use rbf::filter::recordfilter::RecordFilter;
use rbf::layout::Layout;
use rbf::reader::{Reader, ReaderLazyness};
use rbf::record::AsciiMode;

extern crate getopts;
mod args;
use crate::args::CommandLineArguments;

mod util;
use crate::util::{into_field_list, into_rec_map};

mod writer;
use crate::writer::writer::create_writer;

extern crate pbr;
use pbr::{ProgressBar, Units};

#[allow(unused_variables)]
fn main() {
    // get arguments
    let args: Vec<String> = env::args().collect();

    // process arguments
    let mut cmd_args = error_check!(CommandLineArguments::from_args(&args));
    if cmd_args.debug {
        println!("debug: arguments = {:?}", args);
        println!("debug: CommandLineArguments={:?}", cmd_args);
    }

    // tick time
    let now = Instant::now();

    // load layout (suppose only ascii data)
    let mut layout = error_check!(Layout::<AsciiMode>::new(&cmd_args.layout_file));
    let initial_layout_rec_nb = layout.len();
    if cmd_args.debug {
        println!(
            "debug: initial number of layout records = {}",
            initial_layout_rec_nb
        );

        // print out all field types
        for (_, ft) in &layout.ftypes {
            println!("debug: {:?}", ft);
        }
    }

    // in this case, the list of field regexes to filter out records
    let mut record_filter = RecordFilter { expr: Vec::new() };
    let record_filter_list: String;

    if let Some(record_filter_list) = cmd_args.record_filter_list.clone() {
        record_filter = RecordFilter::from(&*record_filter_list);

        // check if field names are valid
        for f in &record_filter.expr {
            if !layout.contains_field(&f.fname) {
                panic!("field name {} not found in layout!", f.fname);
            }
        }

        // debug
        if cmd_args.debug {
            println!("{:?}", record_filter);
        }
    }

    // prune layout if any
    // in this case, the list of rec/fields is provided on the command line
    if let Some(filter_list) = cmd_args.filter_list {
        let rec_map = into_rec_map(&filter_list, ";");
        layout.retain(rec_map);

        // in that case, always set read mode to lazy
        cmd_args.reader_mode = ReaderLazyness::Lazy;
    }
    // while in this case, a file containing rec/fields is provided
    else if let Some(filter_file) = cmd_args.filter_file {
        let mut f = error_check!(File::open(filter_file));
        let mut buffer = String::new();

        error_check!(f.read_to_string(&mut buffer));
        let rec_map = into_rec_map(&buffer, "\n");
        layout.retain(rec_map);

        // in that case, always set read mode to lazy
        cmd_args.reader_mode = ReaderLazyness::Lazy;
    }

    // do we want to always skip some fields for all records?
    if let Some(skip_fields) = cmd_args.skip_fields {
        // remove field names
        layout.remove(into_field_list(&skip_fields));
    }

    // save the number of records as layout will be move out
    let nb_records_created = layout.len();

    // create reader
    let mut reader = Reader::new(&cmd_args.input_file, layout);

    // ignore some lines?
    if let Some(ignore_lines) = cmd_args.ignore_lines {
        // remove field names
        reader.ignore_line = Regex::new(&ignore_lines).unwrap();
    }

    // set reader mode, lazy or strict
    reader.set_lazyness(cmd_args.reader_mode);

    // initialize progress bar
    let mut pb = ProgressBar::new(reader.file_size);
    pb.set_units(Units::Bytes);

    // if verbose is asked, print out some information
    if cmd_args.verbose {
        println!(
            "info: input file is <{}>, size: {} bytes",
            &cmd_args.input_file, reader.file_size
        );
        println!(
            "info: layout file is <{}> with {} record(s)",
            &cmd_args.layout_file, nb_records_created
        );
    }

    // now create writer according to requested output format
    let output_format = match cmd_args.output_format {
        Some(v) => v,
        None => String::from("text"),
    };

    // build output file name depending on format
    let mut writer = create_writer(&output_format, &cmd_args.input_file);

    // loop through records
    while let Some((stats, rec)) = reader.next() {
        // progress bar?
        if cmd_args.progress_bar {
            pb.set(stats.nb_chars_read);
        }

        // only read file, no write? So just loop.
        if cmd_args.only_read {
            continue;
        }

        // samples?
        if let Some(sample_size) = cmd_args.sample_size {
            if sample_size <= stats.nb_lines_read {
                break;
            }
        }

        // filter records ?
        if cmd_args.record_filter_list.is_some() && !rec.is_filter_matched(&record_filter) {
            continue;
        }

        // check pattern?
        if cmd_args.check_pattern {
            for f in &*rec {
                if !f.is_pattern_matched() {
                    println!(
                        "info: <{}:{}> doesn't match the defined field pattern",
                        rec.name, f.name
                    );
                }
            }
        }

        // now it's to write into our writer
        writer.write(rec);
    }

    // finally close our writer
    writer.close();

    // print out results
    let secs = now.elapsed().as_secs();
    println!(
        "info: elapsed time={}.{} secs for {} lines read",
        now.elapsed().as_secs(),
        now.elapsed().subsec_nanos(),
        reader.stats.nb_lines_read
    );

    // in some rare cases, secs could be 0
    if secs != 0 {
        println!(
            "info: average={} lines/s",
            reader.stats.nb_lines_read / secs
        );
    } else {
        println!("info: {} lines read", reader.stats.nb_lines_read);
    }
}
