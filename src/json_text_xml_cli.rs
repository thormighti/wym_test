use std::error::Error;
use std::fs::File;
use std::io;
use xml_builder::{XMLBuilder, XMLElement, XMLVersion};

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

// #[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    //used options incase field is empty, String might be pretty expensive
    _column_a: Option<String>,
    column_b: Option<String>,
    column_c: Option<String>,
    column_d: Option<i32>,
    column_e: Option<i32>,
    _column_f: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum RecordResultFormat {
    PlainText,
    JSON,
    XML,
}

impl RecordResultFormat {
    fn new(result: &str) -> Self {
        match result {
            "text" => Self::PlainText,
            "xml" => Self::XML,
            _ => Self::JSON, // json is deafult
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CsvReader {
    pub records: Vec<Record>,
    pub format: RecordResultFormat,
}

impl CsvReader {
    pub fn new(formet : &str) -> Result<Self, Box<dyn Error>> {
        let mut rdr = csv::Reader::from_reader(io::stdin());

        //store vector of the records
        let mut records = vec![]; //handle this could use itererators somehow

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
        let mut csv_fil = File::create("peter.xml").unwrap();
        let mut xml = XMLBuilder::new()
            .version(XMLVersion::XML1_0)
            .encoding("UTF-8".into())
            .build();
        let mut big_csv = XMLElement::new("BigCSV");
        big_csv.add_attribute("id", "Parent Biggie");

        for (index, rows) in self.records.iter().enumerate() {
            if let (Some(e), Some(d)) = (&rows.column_d, &rows.column_e) {
                let sum_cd = e + d;
                if sum_cd > 100 {
                    if let (Some(b), Some(c)) = (&rows.column_b, &rows.column_c) {
                        match self.format {
                            RecordResultFormat::JSON => {

                                let json_obj_inner_struct = vec![RecordJsonInner {
                                    line_number: index as u32 + 1,
                                    types: "Ok".to_string(),
                                    concat_ab: format!("{} {}", b, c),
                                    sum_cd,
                                }];
                                let convert_to_json =
                                    // serde_json::to_string(&json_obj_inner_struct).unwrap();
                                    serde_json::to_value(json_obj_inner_struct).unwrap();

                                println!("{}", convert_to_json);
                            }
                            RecordResultFormat::PlainText => {
                                println!("{}  {}  {}", index + 1, b, c);
                            }
                            RecordResultFormat::XML => {
                                let xml_objs_struct = RecordJsonInner {
                                    line_number: index as u32 + 1,
                                    types: "Ok".to_string(),
                                    concat_ab: format!("{} {}", b, c),
                                    sum_cd,
                                };
                                //xml builder call

                                // let mut big_csv = XMLElement::new("BigCSV");
                                let mut line_number = XMLElement::new("LineNumber");
                                line_number.add_attribute(
                                    "line_number",
                                    &xml_objs_struct.line_number.to_string(),
                                );
                                big_csv.add_child(line_number).unwrap();
                                let mut types = XMLElement::new("Types");
                                types.add_attribute(
                                    "error_type",
                                    &xml_objs_struct.types.to_string(),
                                );
                                big_csv.add_child(types).unwrap();
                                let mut concatab = XMLElement::new("ErrorMessage");
                                concatab.add_attribute(
                                    "message",
                                    &xml_objs_struct.concat_ab.to_string(),
                                );
                                big_csv.add_child(concatab).unwrap();
                                let mut sumcd = XMLElement::new("sum");
                                sumcd.add_attribute(
                                    "summation",
                                    &xml_objs_struct.sum_cd.to_string(),
                                );
                                big_csv.add_child(sumcd).unwrap();

                                //    let mut writer :Vec<u8> = Vec::new();
                                // xml.generate(&mut csv_fil).unwrap();

                                // println!("{:?}",writer);
                            }
                        }
                    }
                }

                if let (Some(b), Some(c)) = (&rows.column_b, &rows.column_c) {
                    //for unproceesed rows
                    match self.format {
                        RecordResultFormat::JSON => {
                            let json_error_objs_struct = vec![RecordJsonErro {
                                line_number: index as u32 + 1,
                                types: "error".to_string(),
                                error_messages: "This row cant be processed correctly.".to_string(),
                            }];

                            let convert_to_error_json =
                                serde_json::to_value(json_error_objs_struct);
                            // serde_json::to_string(&json_error_objs_struct).unwrap(); // need to handle this too

                            //   let jvec = vec![convert_to_error_json];

                            println!("{}", convert_to_error_json.unwrap());
                        }

                        RecordResultFormat::PlainText => {
                            println!("{}  {}  {}", index + 1, b, c);
                        }
                        RecordResultFormat::XML => {
                            let xml_error_objs_struct = RecordJsonErro {
                                line_number: index as u32 + 1,
                                types: "error".to_string(),
                                error_messages: "This row cant be processed correctly.".to_string(),
                            };

                            let mut line_number = XMLElement::new("LineNumber");
                            line_number.add_attribute(
                                "line_number",
                                &xml_error_objs_struct.line_number.to_string(),
                            );
                            big_csv.add_child(line_number).unwrap();
                            let mut types = XMLElement::new("Types");
                            types.add_attribute("error_type", &xml_error_objs_struct.types);
                            big_csv.add_child(types).unwrap();
                            let mut error_messages = XMLElement::new("ErrorMessage");
                            error_messages
                                .add_attribute("message", &xml_error_objs_struct.error_messages);
                            big_csv.add_child(error_messages).unwrap();

                            // xml.set_root_element(big_csv);

                            // let mut writer :Vec<u8> = Vec::new();

                            // println!("{:?}",writer);
                        }
                    }
                }
            }
        }
        xml.set_root_element(big_csv);
        xml.generate(&mut csv_fil).unwrap();
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
#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_json(){
        let input = ";here is some value 9511;columnB;44;69;";
        let output = r#"
        [{"concat_ab":"here is some value 9511 columnB","line_number":9511,"sum_cd":113,"types":"Ok"}]"#;
        // let record = CsvReader::test_one();

    }
}





























/* Phew!, pretty handy work habdling xml files. used xml builder crates to output xml way to the user
but i wrote it to a file dont want to mess my screen
good idea lets write all my output to file haha. nahhhhhh lets leave it like this for now.lol */