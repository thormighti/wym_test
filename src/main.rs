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
    if let Ok(err) = run() {
        

    test_one(err);

    }
    
      if let  Err(err) = run(){
             println!("error running example: {}", err);
         process::exit(1);

        }
    }




fn parse_column(input: String) -> Record {

    let record_fields: Vec<_> = input.split(";").collect();

    //rows should contain atleast 5 fields

     if record_fields.len() < 5{
         println!("Unexpected number of columns");
     };

    // parsing the c and d column to integers
    let d :i32= record_fields[3].trim().parse().unwrap_or(0);
    let e :i32= record_fields[4].trim().parse().unwrap_or(0);



//Realized some rows had 6 fields and some had 5, had to address them here
if record_fields.len() == 5 {
  Record{
    A: Some(record_fields[0].to_string()),
    B: Some(record_fields[1].to_string()),
    C: Some(record_fields[2].to_string()),
    D: Some(d),
    E: Some(e),
    F: None,

}
}
else {
  Record{
    A: Some(record_fields[0].to_string()),
    B: Some(record_fields[1].to_string()),
    C: Some(record_fields[2].to_string()),
    D: Some(d),
    E: Some(e),
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




type RecordJson = HashMap<String,RecordJsonInner>;

//Checks for sum of c and d if > 100 in test one
fn test_one(records:Vec<Record>){

    for (index,rows) in records.into_iter().enumerate(){
        if let (Some(e), Some(d)) = (&rows.D, &rows.E){
            let sum_cd = e +d;
            if sum_cd  > 100{
                
                if let (Some(b) , Some(c)) = (&rows.B, &rows.C){
                    let json_obj_inner_struct = RecordJsonInner{
                        line_number: index as u32 +1,
                        types: "Ok".to_string(),
                        concat_ab: format!("{} {}", c, d),
                        sum_cd
                    };
                    let convert_to_json = serde_json::to_value(json_obj_inner_struct);

                    println!("{:?}", convert_to_json);
                    // println!("{}  {}  {}",index+1, b,c);
                }
            }

        }
    }
}
/* 

Rows that can be processed correctly with C + D > 100: { "lineNumber":
 <FILE_LINE_NUMBER>, "type": "ok", "concatAB": "<PREVIOUS_AB_CONCAT>", "sumCD": <PREVIOUS_CD_SUM> }

  */