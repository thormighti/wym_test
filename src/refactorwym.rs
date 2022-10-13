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
    fn json_format<'a>(&self, reader: &'a Record) -> & 'static str;
    fn text_formatter<'a>(&self, reader: &'a Record);
    fn xml_formatter<'a>(&self, reader: &'a Record) -> std::fs::File;
    
}



impl OutputFormatter for OkLineOutput {
     fn json_format<'a>(&self, reader: &'a Record) -> &'static str {
        ""
        
    }
    fn text_formatter<'a>(&self, reader: &'a Record)  {
         let  count = 0;
       
        
        if let (Some(c), Some(d)) = (reader.column_c, reader.column_d) {
             let sum_cd = c + d;
                if sum_cd > 100 {
                    if let (Some(_a), Some(b)) = (&reader.column_a, &reader.column_b) {
                                println!("{}  {}  {}", count + 1, b, c)


                }
               
            }
            

        }
        
        
    }
    fn xml_formatter<'a>(&self, reader: &'a Record) -> std::fs::File {
        File::create("peter.xml").unwrap()
        
    }
    fn format_okline(&self, line:OkLineOutput) {
        
    }
    
}

impl OutputFormatter for ErrorLineOutput {
    fn json_format<'a>(&self,reader: &'a Record) -> &'static str {
        ""
        // plain function implementation of json format, complete with for loop at csv reader
         // if args[1] = "json"{
            // do json guys
        //}
        
    }
    fn text_formatter<'a>(&self, reader: &'a Record) {

        let count = 0;
         if let (Some(a), Some(b)) = (&reader.column_a, &reader.column_b) {
            println!("{}  {}  {}", count + 1, a, b);

         }
        
       
        
        
        }
        
    
    fn xml_formatter<'a>(&self, reader: &'a Record) -> std::fs::File {
        File::create("peter.xml").unwrap()
        
    }

    fn format_errorline(&self, line:ErrorLineOutput) {
        
        
    }

    
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    //used options incase field is empty, String might be pretty expensive
    column: Option<String>,
    #[serde(rename = "columnA")]
    column_a: Option<String>,
    #[serde(rename = "columnB")]
    column_b: Option<String>,
    #[serde(rename = "columnC")]
    column_c: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "columnD")]
    column_d: Option<i32>,
    #[serde(rename = "otherColumn")]
    #[serde(deserialize_with = "csv::invalid_option")]
    other_column: Option<String>,
}

pub struct CsvReader{
   state : Box<dyn OutputFormatter> ,// we got to find out the format role
   records: Record

}

impl CsvReader{
       pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        //builds csv files. awesome stuff
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .flexible(true)
            .from_reader(io::stdin());

        let mut count = 0; // track index
        let args: Vec<String> = env::args().collect();

        for result in rdr.deserialize(){
           self.records  = result?;
           self.state.text_formatter(&self.records) // some mess up.lol


        }

        //lets get the arg[1]
        Ok(())
       }
       
}


//when we using the traits, go self.outputformat.methods in usage for the public functions
