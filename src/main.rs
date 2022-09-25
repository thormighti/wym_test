mod json_text_xml_cli;
use json_text_xml_cli::CsvReader;
use std::process;

fn main() {
    let csv_saver = CsvReader::new();
    if let Err(err) = csv_saver {
        println!("{}", err);
        process::exit(1);
    }

    /*  if args.len() > 1 {

        let csv_saver = CsvReader;

        //borrow from the other cmd program

        if let Ok(val) = csv_saver {

            val
        } else {
            println!("error running example");
            process::exit(1);
        }
    } */
}
