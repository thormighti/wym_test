use std::env;
use std::error::Error;
use std::io;
use std::process;
use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;



 #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
 struct Record {
    //used options incase field is empty, String might be pretty expensive
    A: Option<String>,
    B: Option<String>,
    C: Option<String>,
    D: Option<i32>,
    E: Option<i32>,
    F: Option<String>
}

enum RecordResultFormat{
    PlainText,
    JSON,
}

impl RecordResultFormat {
    fn new(result: &str) -> Self{
        match result {
            "text" => Self::PlainText,
            _ => Self::JSON 
            
        }
    }
    
}

fn run() -> Result<Vec<Record>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    //store vector of the records
        let mut records = vec![];
    
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Option<String> = result?;
        // println!("{:?}", record);
    
        if let Some(data) = record{
            let column = parse_column(data);

            records.push(column);   

        }
    }
    Ok(records)

    
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1{
    println!("{:?}", args[1]);
     if let Ok(err) = run() {
        test_one(err);
    }
    if let  Err(err) = run(){
             println!("error running example: {}", err);
         process::exit(1);
        }
}
   
    } 





fn parse_column(input: String) -> Record {

    let record_fields: Vec<_> = input.split(";").collect();

    //rows should contain atleast 5 fields

     if record_fields.len() < 5{
         println!("Unexpected number of columns");
     };

    // parsing the c and d column to integers

    let get_value = |val:&str| -> Option<i32>{
        let theval = val.trim().parse::<i32>();
        if let Ok(val) = theval{
            Some(val)
        }
        else {
            None
        }
    }; 
//Realized some rows had 6 fields and some had 5, had to address them here
if record_fields.len() == 5 {
  Record{
    A: Some(record_fields[0].to_string()),
    B: Some(record_fields[1].to_string()),
    C: Some(record_fields[2].to_string()),
    D: get_value(record_fields[3]),
    E: get_value(record_fields[4]),
    F: None,
}
}
else {
  Record{
    A: Some(record_fields[0].to_string()),
    B: Some(record_fields[1].to_string()),
    C: Some(record_fields[2].to_string()),
    D: get_value(record_fields[3]),
    E: get_value(record_fields[4]),
    F: Some(record_fields[5].to_string()),
}
}

}

 #[derive(Debug, Deserialize,Serialize)]
struct RecordJsonInner{
    line_number : u32,
    types: String,
    concat_ab : String,
    sum_cd : i32
}

 #[derive(Debug, Deserialize,Serialize)]
 struct Record_Json_Erro{
    line_number : u32,
    types: String,
    error_messages: String

 }
// type RecordJson = HashMap<String,RecordJsonInner>;

//Checks for sum of c and d if > 100 in test one
fn test_one(records:Vec<Record>, formet:&str){
    for (index,rows) in records.into_iter().enumerate(){
        if let (Some(e), Some(d)) = (&rows.D, &rows.E){
            let sum_cd = e +d;
            if sum_cd  > 100{
               
                
                if let (Some(b) , Some(c)) = (&rows.B, &rows.C){

                     match RecordResultFormat::new(formet){
                    RecordResultFormat::JSON => {
                         let json_obj_inner_struct = RecordJsonInner{
                        line_number: index as u32 +1,
                        types: "Ok".to_string(),
                        concat_ab: format!("{} {}", c, d),
                        sum_cd
                    };
                    let convert_to_json = serde_json::to_value(json_obj_inner_struct);

                    println!("{:?}", convert_to_json);
                    },
                    RecordResultFormat::PlainText =>  println!("{}  {}  {}",index+1, b,c)
                }
            }

        }
        let json_error_objs_struct = Record_Json_Erro{
            line_number: index as u32 +1,
            types: "error".to_string(),
            error_messages: "This row cant be processed correctly.".to_string(),
        };

        let convert_to_error_json = serde_json::to_value(json_error_objs_struct);
        println!("Error_Json : {:?}", convert_to_error_json);

    }
}
}
    
