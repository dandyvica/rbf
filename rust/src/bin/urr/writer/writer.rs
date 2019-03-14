use crate::writer::csv::CsvWriter;
use crate::writer::text::TextWriter;
use rbf::record::{AsciiMode, Record, UTF8Mode};

pub trait Writer {
    fn new(metadata: &str) -> Self
    where
        Self: Sized;
    fn write(&mut self, rec: &Record<AsciiMode>);
    fn close(&self);
}

/// Convenient creation of a Writer
pub fn create_writer(from: &str, input_file: &str) -> Box<Writer> {
    match from {
        "text" => Box::new(TextWriter::new(input_file)),
        "csv" => Box::new(CsvWriter::new(input_file)),
        unknown_type @ _ => panic!("<{}> is not a valid output format", unknown_type),
    }
}
