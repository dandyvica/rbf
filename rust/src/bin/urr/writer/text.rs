use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

use crate::writer::writer::Writer;
use rbf::record::{AsciiMode, Record, UTF8Mode};

pub struct TextWriter {
    last_recname: String,
    buffer: BufWriter<File>,
}

impl Writer for TextWriter {
    fn new(input_file: &str) -> TextWriter {
        // build output file name
        let output_file = input_file.to_owned() + ".txt";

        // open file for reading
        let file = match File::create(&output_file) {
            // if ok, create a new BufReader to read the file line by line
            Ok(f) => f,
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", output_file, why.description()),
        };

        let buffer = BufWriter::new(file);
        TextWriter {
            last_recname: String::new(),
            buffer: buffer,
        }
    }

    fn close(&self) {}

    #[allow(unused_must_use)]
    fn write(&mut self, rec: &Record<AsciiMode>) {
        // build header from field names only if not the same record than before
        if self.last_recname != rec.name {
            self.buffer.write(b"\n");
            let header: Vec<_> = rec
                .flist
                .iter()
                .map(|f| format!("{:length$} ", f.name, length = f.cell_size))
                .collect();
            self.buffer.write(&header.join("|").as_bytes());
            self.buffer.write(b"\n");

            // last rec name is now current
            self.last_recname = rec.name.clone();
        }

        // now data
        let data: Vec<_> = rec
            .flist
            .iter()
            .map(|f| format!("{:length$} ", f.value(), length = f.cell_size))
            .collect();
        self.buffer.write(&data.join("|").as_bytes());
        self.buffer.write(b"\n");
    }
}
