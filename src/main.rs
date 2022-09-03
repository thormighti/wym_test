use std::error::Error;
use std::io;
use std::process;

use serde::Deserialize;



 #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
 struct Record {
    A: Option<String>,
    B: Option<String>,
    C: Option<String>,
    D: Option<i32>,
    E: Option<i32>,
    F: Option<String>
}

fn example() -> Result<Vec<Record>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
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
    if let Ok(err) = example() {
        

    test_one(err);

    }
    
      if let  Err(err) = example(){
             println!("error running example: {}", err);
         process::exit(1);

        }
    }




fn parse_column(input: String) -> Record {

    let record_fields: Vec<_> = input.split(";").collect();

     if record_fields.len() < 5{
         println!("Unexpected number of columns");
     };

    let d :i32= record_fields[3].trim().parse().unwrap_or(0);
    let e :i32= record_fields[4].trim().parse().unwrap_or(0);




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

//Checks for sum of c and d if > 100
fn test_one(records:Vec<Record>){

    for rows in &records{
        if let (Some(e), Some(d)) = (&rows.D, &rows.E){
            if e + d  > 100{
                
                if let (Some(b) , Some(c)) = (&rows.B, &rows.C){
                    println!("{}{}", b,c);
                }
            }

        }
    }
}
