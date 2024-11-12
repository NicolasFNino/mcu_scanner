use std::{fs::File, io::{BufRead, BufReader, Lines, Result}, path::Path};

// TODO: The structures should be much more complex
//
// We need to have a criteria to 'discard' a signature based on each field
// examples: 1. file size is negative
//           2. a field value is not valid for the signature that is supposed to be a part of
//
extern crate crc;
const X25: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);

use regex::Regex;

#[derive(Debug)]
pub struct Signature{
    fields: Vec<Field>,
}

#[derive(Debug)]
#[derive(Clone)]
struct Field {
    position: usize,
    value_type: FieldType,
    constraint: String,
    description: String
}

#[derive(Debug)]
#[derive(Clone)]
enum FieldType {
    Str,
    Date,
    Regex,
    Byte,
    Short,
    Long,
    Default
}

impl Default for Field {
    fn default() -> Self {
        Field {
            position: 0,
            value_type: FieldType::Default,
            constraint: String::from("Unknown"),
            description: String::from("Unknown"),
        }
    }
}


pub fn verify_file(contents: Vec<u8>) -> Vec<Vec<(String, String)>>{
    println!("\n3. Verifing the contents of the input file:");
    let mut signature_matches: Vec<Vec<(String, String)>> = Vec::new();
    let signatures = read_signatures();

    for entry in signatures {
        let mut mutable_content = contents.clone();
        match_signature(entry, &mut mutable_content, &mut signature_matches);
    }

    // let calc_crc = calculate_crc(&contents);
    // let expect_crc: u16 = 0x906e;

    // if calc_crc == expect_crc {
    //     println!("CRC check passed!");
    // } else {
    //     println!("CRC check failed!");
    //     //ToD: handle this
    //     return Vec::new();
    // }

    //signatures
    signature_matches
}

fn match_signature(entry: Signature, file_content: &mut Vec<u8>, list_matches: &mut Vec<Vec<(String, String)>>) {
    let mut is_valid = true;
    let mut current_match: Vec<(String, String)> = Vec::new();
    for field in entry.fields {
        if !is_valid {
            break;
        }
        let pos = field.position;
        let vtype = field.value_type;
        let value = field.constraint;
        let desc = field.description;
        match vtype {
            FieldType::Str => {
                println!("Signature field is a string:");
                let end_index = value.len() + pos; 
                if end_index < file_content.len() {
                    let slice = &file_content[pos..end_index];
                    for (index, character) in slice.iter().enumerate() {
                        if *character == value.as_bytes()[index] {
                            // println!("Char match!");
                        } else {
                            println!("Signature match failed:\n\t{} != {}", *character, value.as_bytes()[index]);
                            is_valid = false;
                        }
                    }
                    if is_valid {
                        current_match.push((desc, value));
                    }
                } else {
                    println!("There was something wrong!");
                    is_valid = false;
                }
            },
            FieldType::Byte => {
                println!("Signature field is a byte:");
                if file_content[pos] == value.as_bytes()[0] {
                    println!("Byte match!");
                } else {
                    println!("Not a match!");
                    is_valid = false;
                }
            },
            FieldType::Short => {
                println!("Signature field is a short:")
            }
            FieldType::Long => {
                println!("Signature field is a long:");
                if value == 'x'.to_string() {
                    println!("Only for displaying purposes");
                }
                let in_file = &file_content[pos..pos+4];
                if desc.trim() == "File size," {
                    // Convert the slice into an array of 4 bytes
                    let size = in_file.try_into().expect("Failed to convert slice to array");

                    // Convert the bytes to a 32-bit unsigned integer (little-endian)
                    let size = u32::from_le_bytes(size);
                    println!("Size in file: {}", size);

                    // TODO: Check file size

                    //Add it to the curretn signature match fields
                    current_match.push((desc, format!("{} bytes", size)));
                }         
            }
            _ => {

            }
        }
    }
    if is_valid {
        list_matches.push(current_match);
    }
}

fn read_signatures() -> Vec<Signature> {
    let mut results = Vec::new();

    let re = Regex::new(r"\s+").unwrap();

    if let Ok(lines) = read_lines("magic/vendors_to_test") {
        let mut fields_to_add: Vec<Field> = Vec::new();
        for line in lines.flatten() {

            let mut field = Field {
                ..Default::default()
            };

            // if line.as_str().starts_with("#") || line.as_str().starts_with("\n") || line.as_str().starts_with(" ") {
            if line.as_str().starts_with("#") || line.len() == 0 {
                // Ignore the lines that start with #
                if line.as_str().starts_with(" ") || line.as_str().starts_with("\n") || line.len() == 0 {
                    println!("EOS LINE:\n{}", line);
                    if fields_to_add.len() > 0 {
                        let current_signature = Signature {
                            fields: fields_to_add.clone()
                        };
                        println!("{:#?}", current_signature);
                        
                        results.push(current_signature);
                        fields_to_add.clear();
                    }

                } else {
                    println!("ELSE: {}", line)
                }
                
            } else {
                // Here is where the signature population should happen
                println!("GOOD LINE:\n{}", line);
                //println!("Line:\n{:#?}", line.as_bytes());

                let split = re.splitn(&line, 4);

                for (index, item) in split.enumerate() {
                    //println!("\n==========\nITEM: {:#?}\n==========\n", item.as_bytes());
                    println!("\n==========\nITEM: {}\n==========\n", item);
                    match index {
                        0 => {
                            let mut num: usize  = 0;
                            if item.starts_with(">") {
                                let re = Regex::new(r">\s*(\d+)").unwrap();
                                if let Some(captures) = re.captures(item.trim()) {
                                    // Extract the number
                                    let number = &captures[1];
                                    println!("Matched number: {}", number);
                                    num = number.trim().parse().expect("Not a valid number!");
                                    
                                } else {
                                    println!("No match found");
                                }
                                
                            } else {
                                num = item.trim().parse().expect("Not a valid number!");
                            }
                            field.position = num;
                            
                        }, 
                        1 => {
                            match item {
                                "byte" => { field.value_type = FieldType::Byte; },
                                "short" => { field.value_type = FieldType::Short; },
                                "long" => { field.value_type = FieldType::Long; },
                                "string" => { field.value_type = FieldType::Str; },
                                "Date" => { field.value_type = FieldType::Date; },
                                "Regex" => { field.value_type = FieldType::Regex; },
                                &_ => { field.value_type = FieldType::Default; },
                            }
                        }, 
                        2 => {
                            field.constraint = item.to_string();
                        },
                        _ => {
                            field.description = item.to_string();
                        }
                    }
                }
                fields_to_add.push(field);
            }
        }
    }

    results
}

fn calculate_crc(contents: &[u8]) -> u16 {
    X25.checksum(contents)
}

//need to make sure this signature is correct
pub fn verify_with_size(file_path: &str, expected_size: usize) -> std::io::Result<bool> {
    let file = std::fs::read(file_path)?;
    if file.len() != expected_size {
        println!("file size doesnt match expected size");
        return Ok(false);
    } //if
    let checksum = X25.checksum(&file);
    println!("checksum: {:#X}", checksum);
    Ok(true)
    //implementation for checking checksum but idk the actual checksum value we need to compare it to
    // if let Some(expected) = expected_checksum {
    //    println!("checksum doesnt match");
    //    return Ok(false);
    //} //if
    //Ok(true);
} //verify_with_size

pub fn print_data(sig_matches: Vec<Vec<(String, String)>>) {
    println!("\n4. This is the information that you are interested in: ");
    println!("{:#?}", sig_matches);
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}