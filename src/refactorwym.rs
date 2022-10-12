use std::{fs::File, env, io, error::Error};

use serde::{Deserialize,Serialize};
use xml_builder::{XMLBuilder, XMLElement, XMLVersion};
//states are OkLine and ErrorLineoutput
#[derive(Debug, Deserialize, Serialize)]
pub struct OkLineOutput{
     line_number: u32,
    types: String,
    concat_ab: String,
    sum_cd: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorLineOutput{
    line_number: u32,
    types: String,
    error_messages: String,

}

trait OutputFormatter {
    fn format_okline(&self, line:OkLineOutput){
       () // default value
    }
    fn format_errorline(&self, line:ErrorLineOutput){
        ()
    }
    fn json_format(&self) -> & 'static str;
    fn text_formatter(&self) -> &'static str;
    fn xml_formatter(&self) -> std::fs::File;
    
}



impl OutputFormatter for OkLineOutput {
     fn json_format(&self) -> &'static str {
        ""
        
    }
    fn text_formatter(&self) -> &'static str {
        ""
        
    }
    fn xml_formatter(&self) -> std::fs::File {
        File::create("peter.xml").unwrap()
        
    }
    fn format_okline(&self, line:OkLineOutput) {
        
    }
    
}

impl OutputFormatter for ErrorLineOutput {
    fn json_format(&self) -> &'static str {
        ""
        // plain function implementation of json format, complete with for loop at csv reader
        
    }
    fn text_formatter(&self) -> &'static str {
        ""
        
    }
    fn xml_formatter(&self) -> std::fs::File {
        File::create("peter.xml").unwrap()
        
    }

    fn format_errorline(&self, line:ErrorLineOutput) {
        
    }

    
}
pub struct CsvReader{
    format : Box<dyn OutputFormatter> // we got to find out the format role
}

impl CsvReader{
       pub fn run() -> Result<(), Box<dyn Error>> {
        //builds csv files. awesome stuff
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .flexible(true)
            .from_reader(io::stdin());

        let mut count = 0; // track index
        let args: Vec<String> = env::args().collect();

        //lets get the arg[1]
        Ok(())
       }
}


//when we using the traits, go self.outputformat.methods in usage for the public functions
