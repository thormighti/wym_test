use std::fs::File;

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

pub struct CsvReader{
    format : Box<dyn OutputFormatter>
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
