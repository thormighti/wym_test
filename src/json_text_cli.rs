
use std::error::Error;
use std::io;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
struct RecordJsonInner {
    line_number: u32,
    types: String,
    concat_ab: String,
    sum_cd: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct RecordJsonErro {
    line_number: u32,
    types: String,
    error_messages: String,
}

#[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct Record {
    //used options incase field is empty, String might be pretty expensive
    _column_a: Option<String>,
    column_b: Option<String>,
    column_c: Option<String>,
    column_d: Option<i32>,
    column_e: Option<i32>,
    _column_f: Option<String>,
}

pub enum RecordResultFormat {
    PlainText,
    JSON,
}

impl RecordResultFormat {
    fn new(result: &str) -> Self {
        match result {
            "text" => Self::PlainText,
            _ => Self::JSON, // json is deafult
        }
    }
}

pub struct CsvReader {
   pub records: Vec<Record>,
   pub format: RecordResultFormat,
}

impl CsvReader {
    pub fn new(formet: &str) -> Result<Self, Box<dyn Error>> {
        let mut rdr = csv::Reader::from_reader(io::stdin());

        //store vector of the records
        let mut records = vec![];

        for result in rdr.deserialize() {
            // Notice that we need to provide a type hint for automatic
            // deserialization.
            let record: Option<String> = result?;
            // println!("{:?}", record);

            if let Some(data) = record {
                let column = Self::parse_column(data);

                records.push(column);
            }
        }
        let format = RecordResultFormat::new(formet);
        Ok(Self { records, format })
    }

   pub fn test_one(&self) {
        for (index, rows) in self.records.iter().enumerate() {
            if let (Some(e), Some(d)) = (&rows.column_d, &rows.column_e) {
                let sum_cd = e + d;
                if sum_cd > 100 {
                    if let (Some(b), Some(c)) = (&rows.column_b, &rows.column_c) {
                        match self.format {
                            RecordResultFormat::JSON => {
                                let json_obj_inner_struct = RecordJsonInner {
                                    line_number: index as u32 + 1,
                                    types: "Ok".to_string(),
                                    concat_ab: format!("{} {}", c, d),
                                    sum_cd,
                                };
                                let convert_to_json = serde_json::to_value(json_obj_inner_struct);

                                println!("{:?}", convert_to_json);
                            }
                            RecordResultFormat::PlainText => {
                                println!("{}  {}  {}", index + 1, b, c)
                            }
                        }
                    }
                }
               
                if let (Some(b), Some(c)) = (&rows.column_b, &rows.column_c) {

                //for unproceesed rows
                match self.format {
                    RecordResultFormat::JSON => {
                          let json_error_objs_struct = RecordJsonErro {
                    line_number: index as u32 + 1,
                    types: "error".to_string(),
                    error_messages: "This row cant be processed correctly.".to_string(),
                };

                let convert_to_error_json = serde_json::to_value(json_error_objs_struct);
                println!("Error_Json : {:?}", convert_to_error_json);
                    }

                    RecordResultFormat::PlainText => {
                        println!("{}  {}  {}", index + 1, b, c)

                    }
                    
                }

            } 

            }
        }
    }

    fn parse_column(input: String) -> Record {
        let record_fields: Vec<_> = input.split(";").collect();

        //rows should contain atleast 5 fields

        if record_fields.len() < 5 {
            println!("Unexpected number of columns");
        };

        // parsing the c and d column to integers

        let get_value = |val: &str| -> Option<i32> {
            let theval = val.trim().parse::<i32>();
            if let Ok(val) = theval {
                Some(val)
            } else {
                None
            }
        };
        //Realized some rows had 6 fields and some had 5, had to address them here
        if record_fields.len() == 5 {
            Record {
                _column_a: Some(record_fields[0].to_string()),
                column_b: Some(record_fields[1].to_string()),
                column_c: Some(record_fields[2].to_string()),
                column_d: get_value(record_fields[3]),
                column_e: get_value(record_fields[4]),
                _column_f: None,
            }
        } else {
            Record {
                _column_a: Some(record_fields[0].to_string()),
                column_b: Some(record_fields[1].to_string()),
                column_c: Some(record_fields[2].to_string()),
                column_d: get_value(record_fields[3]),
                column_e: get_value(record_fields[4]),
                _column_f: Some(record_fields[5].to_string()),
            }
        }
    }
}