use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

use crate::writer::writer::Writer;
use rbf::record::{AsciiMode, Record, UTF8Mode};

pub struct CsvWriter {
    buffer: BufWriter<File>,
}

impl Writer for CsvWriter {
    fn new(input_file: &str) -> CsvWriter {
        // build output file name
        let output_file = input_file.to_owned() + ".csv";

        // open file for reading
        let file = match File::create(&output_file) {
            // if ok, create a new BufReader to read the file line by line
            Ok(f) => f,
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", output_file, why.description()),
        };

        let buffer = BufWriter::new(file);

        CsvWriter { buffer: buffer }
    }

    fn close(&self) {}

    #[allow(unused_must_use)]
    fn write(&mut self, rec: &Record<AsciiMode>) {
        let line: Vec<_> = rec.flist.iter().map(|f| f.value().to_string()).collect();
        self.buffer.write(&line.join(";").as_bytes());
        self.buffer.write(b"\n");
    }
}
