use std::env;
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
    pub format: RecordResultFormat,
}

impl CsvReader {
    pub fn new() -> Result<(), Box<dyn Error>> {
        //builds csv files. awesome stuff
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .flexible(true)
            .from_reader(io::stdin());

        let mut count = 0; // track index
        let args: Vec<String> = env::args().collect();

        //lets get the arg[1]

        let format = RecordResultFormat::new(&args[1]);

        //to create the xml file
        let mut csv_fil = File::create("peter.xml").unwrap();
        let mut xml = XMLBuilder::new()
            .version(XMLVersion::XML1_0)
            .encoding("UTF-8".into())
            .build();
        let mut big_csv = XMLElement::new("BigCSV");
        big_csv.add_attribute("id", "Parent Biggie");

        //to here

        for result in rdr.deserialize() {
            let record: Record = result?;

            if let (Some(c), Some(d)) = (&record.column_c, record.column_d) {
                let sum_cd = c + d;
                if sum_cd > 100 {
                    if let (Some(a), Some(b)) = (&record.column_a, &record.column_b) {
                        match format {
                            RecordResultFormat::JSON => {
                                let json_obj_inner_struct = vec![RecordJsonInner {
                                    line_number: count as u32 + 1,
                                    types: "Ok".to_string(),
                                    concat_ab: format!("{} {}", a, b),
                                    sum_cd,
                                }];
                                let convert_to_json =
                                    // serde_json::to_string(&json_obj_inner_struct).unwrap();
                                    serde_json::to_value(json_obj_inner_struct).unwrap();

                                println!("{}", convert_to_json);
                            }
                            RecordResultFormat::PlainText => {
                                println!("{}  {}  {}", count + 1, b, c);
                            }
                            RecordResultFormat::XML => {
                                let xml_objs_struct = RecordJsonInner {
                                    line_number: count as u32 + 1,
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

                // for those with errors

                if let (Some(a), Some(b)) = (&record.column_a, &record.column_b) {
                    match format {
                        RecordResultFormat::JSON => {
                            let json_error_objs_struct = vec![RecordJsonErro {
                                line_number: count as u32 + 1,
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
                            println!("{}  {}  {}", count + 1, a, b);
                        }
                        RecordResultFormat::XML => {
                            let xml_error_objs_struct = RecordJsonErro {
                                line_number: count as u32 + 1,
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
                        }
                    }
                }
            }

            count += 1;
        }
        xml.set_root_element(big_csv);
        xml.generate(&mut csv_fil).unwrap();

        Ok(())
    }
}
