mod json_text_xml_cli;
use std::env;
use json_text_xml_cli::CsvReader;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let csv_saver = CsvReader::new(&args[1]);

        if let Ok(val) = csv_saver {
            val.test_one();
        } else {
            println!("error running example");
            process::exit(1);
        }
    }
}
