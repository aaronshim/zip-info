#![feature(custom_derive, proc_macro)]

extern crate docopt;
extern crate glob;
extern crate rustc_serialize;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate zip;

use docopt::Docopt;

mod zip_info;
mod flat_writer;
mod json_writer;

use zip_info::WriteZipInfo;

/// The Docopt usage string
const USAGE: &'static str = "
Usage: zi [-j | -p] [--exclude=<glob>] <path> ...
       zi --help

zi presents information about Zip archives.

Common options:
    -h, --help         Show this usage message.
    -j, --json         Structure the output in JSON
    -p, --pretty-json  Structure the output in easy-to-read JSON
    --exclude=<glob>   Ignore objects in the archives whose name
                       is like this glob pattern.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_path: Vec<String>,
    flag_json: bool,
    flag_pretty_json: bool,
    flag_exclude: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    // XXX We divide up the display functionality here since
    // the original flat writer works differently than the JSON
    // writer. Ultimately, the JSON writer should be a generic
    // serializer for different output modes, so this will be a
    // lot cleaner.
    if !(args.flag_json || args.flag_pretty_json) {
        let mut multiwriter = flat_writer::MultiArchiveFlatWriter::new(
            args.arg_path.as_slice()
        );

        println!("{}", multiwriter.write_zip_info(args.flag_exclude.as_str()));
    } else {
        // Convert String to &str for json printing since
        // Docopt appears not to be able to handle Vec<&str>:
        let mut path_names: Vec<&str> = Vec::new();

        for path in &args.arg_path {
            path_names.push(path.as_str());
        }

        let s = match args.flag_json {
            true => json_writer::zips_to_json,
            _ => json_writer::zips_to_json_pretty,
        };

        println!("{}", s(path_names.as_slice(), args.flag_exclude.as_str()));
    }
}
