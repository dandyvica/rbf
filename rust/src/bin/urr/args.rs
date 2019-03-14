use getopts::{Fail, Matches, Options};
use std::str::FromStr;

use rbf::reader::ReaderLazyness;

// help text
static HELP_MSG: &'static str = r#"
NAME
       urr - read a record-based file and convert it to a known format

SYNOPSIS
       urr -i file - l layout [-o format] [-f file] [-c] [-O] [--br] [--ff] [--fl] [--fr] [-r file] [-s n] [-v] [-p] [-h] [--dup] [--of file] [--conf configfile] [--postsql sqlfile] [--presql sqlfile] [--ua]

DESCRIPTION
       This program is aimed at reading a record-based file and converting it to a human-readable format.

OPTIONS
       -b : don't write output file but just read input file (benchmark).

       --check : check whether field patterns are matched.

       --debug : additional verbosity.       

       --fl records/fields list : only write selected records/fields (filter fields).

       --ff file : full path/name of filter field file.

       --fr condition : only include records matching condition (filter records).

       -h (--help) : this help.
       
       -i file : full path/name of the file to be read and converted.

       -l layout : name of the input file layout.

       -o format : name of the output format. Possible values are: 

        text:     an Ascii file, one line per record with field names (this is the default output format)
        csv:      a text file, one record per line, fields separated by the ';'
        ident:    same file format than the input file, but matching input parameters
        sqlite3:  a sqlite3 database file, one table per record
        tag:      a text file, one record per line, all fields tagged with the following format: field_name = "field_value"
        excel:    a Microsoft Excel (2007 and above) workbook file format, one record per line, one field per cell (one worksheet)

       -p : print out progress bar.

       --rf fields : list of field regexes to filter out records.

       --raw : use raw values instead of blank stripped values

       -s n : only convert the n-first records.

       --strict: if a record if not found is the layout, exit the program.

       -v : print out options (verbose).



"#;

// list of all options
type OptionLongShort = (&'static str, &'static str, &'static str);

// Required options
const OPTION_INPUT_FILE: OptionLongShort = ("i", "input", "");
const OPTION_LAYOUT_FILE: OptionLongShort = ("l", "layout", "");

// Optional options
const OPTION_OUTPUT_FORMAT: OptionLongShort = ("o", "output", "OUTPUT");
const OPTION_HELP: OptionLongShort = ("-h", "--help", "HELP");
const OPTION_BENCHMARK: OptionLongShort = ("b", "bench", "");
const OPTION_PROGRESS_BAR: OptionLongShort = ("p", "progress", "");
const OPTION_VERBOSE: OptionLongShort = ("v", "verbose", "");
const OPTION_DEBUG: OptionLongShort = ("", "debug", "");
const OPTION_STRICT_MODE: OptionLongShort = ("t", "strict", "");
const OPTION_LAZY_MODE: OptionLongShort = ("z", "lazy", "");
const OPTION_SAMPLE: OptionLongShort = ("s", "sample", "SAMPLE");
const OPTION_CHECK_PATTERN: OptionLongShort = ("c", "check", "");
const OPTION_FIELD_FILTER_LIST: OptionLongShort = ("", "fl", "FIELDS");
const OPTION_FIELD_FILTER_FILE: OptionLongShort = ("", "ff", "FIELDS");
const OPTION_RECORD_FILTER_LIST: OptionLongShort = ("", "rf", "RECORD_FILTER");
const OPTION_ASCII_MODE: OptionLongShort = ("", "ascii", "");
const OPTION_UTF8_MODE: OptionLongShort = ("", "utf8", "");
const OPTION_SKIP_FIELDS: OptionLongShort = ("", "skip", "FIELDS");
const OPTION_IGNORE_LINE: OptionLongShort = ("", "ignore", "REGEX");

#[derive(Debug)]
pub struct CommandLineArguments {
    pub input_file: String,
    pub layout_file: String,
    pub only_read: bool,
    pub progress_bar: bool,
    pub verbose: bool,
    pub debug: bool,
    pub sample_size: Option<u64>,
    pub reader_mode: ReaderLazyness,
    pub check_pattern: bool,
    pub filter_list: Option<String>,
    pub filter_file: Option<String>,
    pub skip_fields: Option<String>,
    pub ignore_lines: Option<String>,
    pub output_format: Option<String>,
    pub record_filter_list: Option<String>,
}

impl CommandLineArguments {
    /// Sets the optional option according to OptionLongShort type: first element is the short option
    /// string, second is the long one.
    fn set_optional_opt(opts: &mut Options, long_short: &OptionLongShort) {
        // set option with no help msg, depending on third element: if present, it means
        // this option required an argument
        if long_short.2 == "" {
            // long option that is optional and does not take an argument
            opts.optflag(long_short.0, long_short.1, "");
        } else {
            // long option that is optional and takes an argument
            opts.optopt(long_short.0, long_short.1, "", long_short.2);
        }
    }

    /// Extracts the optional argument if option is provided.
    fn extract_optional_arg<T: FromStr>(matches: &Matches, opt: &'static str) -> Option<T> {
        // option not provided?
        if !matches.opt_present(opt) {
            return None;
        }

        // now something to process
        let arg = match matches.opt_str(opt) {
            Some(v) => v,
            None => panic!("fatal: option <{}> provided but no argument", opt),
        };

        // now try to convert argument
        let conv = match arg.parse::<T>() {
            Ok(v) => v,
            Err(_) => panic!(
                "fatal: option argument <{}> for option <{}> provided, but conversion failed!",
                arg, opt
            ),
        };

        Some(conv)
    }

    pub fn from_args(args: &Vec<String>) -> Result<CommandLineArguments, Fail> {
        // get arguments
        if args.len() == 1
            || (args.len() == 2 && (args[1] == OPTION_HELP.0 || args[1] == OPTION_HELP.1))
        {
            println!("{}", HELP_MSG);
            ::std::process::exit(1);
        }

        // define our new set of options
        let mut opts = Options::new();

        // mandatory options
        opts.reqopt(OPTION_INPUT_FILE.0, OPTION_INPUT_FILE.1, "", "INFILE");
        opts.reqopt(OPTION_LAYOUT_FILE.0, OPTION_LAYOUT_FILE.1, "", "LAYOUT");

        // optional options
        CommandLineArguments::set_optional_opt(&mut opts, &OPTION_BENCHMARK);
        CommandLineArguments::set_optional_opt(&mut opts, &OPTION_VERBOSE);
        CommandLineArguments::set_optional_opt(&mut opts, &OPTION_DEBUG);
        CommandLineArguments::set_optional_opt(&mut opts, &OPTION_PROGRESS_BAR);
        CommandLineArguments::set_optional_opt(&mut opts, &OPTION_STRICT_MODE);
        CommandLineArguments::set_optional_opt(&mut opts, &OPTION_LAZY_MODE);
        CommandLineArguments::set_optional_opt(&mut opts, &OPTION_SAMPLE);
        CommandLineArguments::set_optional_opt(&mut opts, &OPTION_FIELD_FILTER_LIST);
        CommandLineArguments::set_optional_opt(&mut opts, &OPTION_FIELD_FILTER_FILE);
        CommandLineArguments::set_optional_opt(&mut opts, &OPTION_CHECK_PATTERN);
        CommandLineArguments::set_optional_opt(&mut opts, &OPTION_SKIP_FIELDS);
        CommandLineArguments::set_optional_opt(&mut opts, &OPTION_IGNORE_LINE);
        CommandLineArguments::set_optional_opt(&mut opts, &OPTION_OUTPUT_FORMAT);
        CommandLineArguments::set_optional_opt(&mut opts, &OPTION_RECORD_FILTER_LIST);

        // process arguments
        let matches = opts.parse(&args[1..])?;

        // check lazyness
        let mut reader_mode = if matches.opt_present(OPTION_STRICT_MODE.0) {
            ReaderLazyness::Strict
        } else {
            ReaderLazyness::Lazy
        };
        reader_mode = if matches.opt_present(OPTION_LAZY_MODE.0) {
            ReaderLazyness::Lazy
        } else {
            ReaderLazyness::Strict
        };

        // save arguments to struct
        Ok(CommandLineArguments {
            input_file: matches.opt_str(OPTION_INPUT_FILE.0).unwrap(),
            layout_file: matches.opt_str(OPTION_LAYOUT_FILE.0).unwrap(),
            only_read: matches.opt_present(OPTION_BENCHMARK.0),
            progress_bar: matches.opt_present(OPTION_PROGRESS_BAR.0),
            verbose: matches.opt_present(OPTION_VERBOSE.0),
            debug: matches.opt_present(OPTION_DEBUG.1),
            sample_size: CommandLineArguments::extract_optional_arg::<u64>(
                &matches,
                OPTION_SAMPLE.0,
            ),
            reader_mode: reader_mode,
            check_pattern: matches.opt_present(OPTION_CHECK_PATTERN.0),
            filter_list: CommandLineArguments::extract_optional_arg::<String>(
                &matches,
                OPTION_FIELD_FILTER_LIST.1,
            ),
            filter_file: CommandLineArguments::extract_optional_arg::<String>(
                &matches,
                OPTION_FIELD_FILTER_FILE.1,
            ),
            skip_fields: CommandLineArguments::extract_optional_arg::<String>(
                &matches,
                OPTION_SKIP_FIELDS.1,
            ),
            ignore_lines: CommandLineArguments::extract_optional_arg::<String>(
                &matches,
                OPTION_IGNORE_LINE.1,
            ),
            output_format: CommandLineArguments::extract_optional_arg::<String>(
                &matches,
                OPTION_OUTPUT_FORMAT.1,
            ),
            record_filter_list: CommandLineArguments::extract_optional_arg::<String>(
                &matches,
                OPTION_RECORD_FILTER_LIST.1,
            ),
        })
    }
}
