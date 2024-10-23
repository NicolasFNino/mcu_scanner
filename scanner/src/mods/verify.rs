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


pub fn verify_file(contents: Vec<u8>) -> Vec<Signature>{
    println!("\n3. Verifing the contents of the input file:");
    let signatures = read_signatures();

    for entry in signatures {
        match_signature(entry, &contents);
    }

    let calc_crc = calculate_crc(&contents);
    let expect_crc: u16 = 0x906e;

    if calc_crc == expect_crc {
        println!("CRC check passed!");
    } else {
        println!("CRC check failed!");
        //ToD: handle this
    }

    //signatures
    Vec::new()
}

fn match_signature(entry: Signature, file_content: &Vec<u8>) {
    let mut is_valid = true;
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
                let end_index = value.len() + pos; 
                if end_index < file_content.len() {
                    let slice = &file_content[pos..end_index];
                    for (index, character) in slice.iter().enumerate() {
                        if *character == value.as_bytes()[index] {
                            println!("Char match!")
                        } else {
                            println!("Signature match failed:\n\t{} != {}", *character, value.as_bytes()[index]);
                            is_valid = false;
                        }
                    }
                } else {
                    println!("There was something wrong!");
                    is_valid = false;
                }
            },
            _ => {

            }
        }
    }
    if is_valid {
        
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

fn verify_with_size() -> bool {
    true
}

pub fn print_data(sig_matches: Vec<Signature>) {
    println!("\n4. This is the information that you are interested in: ");
    println!("{:#?}", sig_matches);
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}