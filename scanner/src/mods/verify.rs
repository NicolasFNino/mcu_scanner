use std::{fs::File, io::{BufRead, BufReader, Lines, Result}, path::Path};

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

// Entry point of this file. Takes a file and outputs the information extracted from the signature
pub fn verify_file(file_name: String, contents: Vec<u8>) -> Vec<Vec<(String, String)>>{
    println!("\n3. Verifing the contents of the input file:");
    let mut signature_matches: Vec<Vec<(String, String)>> = Vec::new();
    let signatures = read_signatures();

    for entry in signatures {
        let mut mutable_content = contents.clone();
        match_signature(&file_name, entry, &mut mutable_content, &mut signature_matches);
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

fn parse_description_str(description: &str) -> i32 {
    if description.contains("strlen:") {
        let mut result: Option<i32> = None;
        let re = Regex::new(r"\{strlen:(\d+)\}").unwrap();
        if let Some(captures) = re.captures(description) {
            result = captures.get(1).map(|m| m.as_str().parse().unwrap());
        } else {
            result = None;
        }

        match result {
            Some(number) => { return number },
            None => { return 0 },
        }
    }
    return 0;
}

// Takes a signature and file content and extracts the relevant information
fn match_signature(file_name: &String, entry: Signature, file_content: &mut Vec<u8>, list_matches: &mut Vec<Vec<(String, String)>>) {
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
                // println!("Signature field is a string: {}", value);
                if value.trim() == "x" {

                    let end_index = parse_description_str(&desc);

                    // println!("Only for displaying purposes");
                    if desc.trim() == "OTA header string," {
                        let string = String::from_utf8_lossy(&file_content[pos..pos+32]);
                        current_match.push((desc, string.to_string()));
                    } else {
                        let string = String::from_utf8_lossy(&file_content[pos..pos+end_index as usize]);
                        current_match.push((desc, string.to_string()));
                    }
                    
                } else {

                    let end_index = value.len() + pos; 
                    if end_index < file_content.len() {
                        let slice = &file_content[pos..end_index];
                        for (index, character) in slice.iter().enumerate() {
                            if *character == value.as_bytes()[index]{
                                // println!("Char match!");
                            } else {
                                // println!("Signature match failed:\n\t{} != {}", *character, value.as_bytes()[index]);
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
                }
                
            },
            FieldType::Byte => {
                // println!("Signature field is a byte:");
                if value.trim() == "x" {
                    // println!("Only for displaying purposes");
                    current_match.push((desc, format!("{}", file_content[pos])));
                } else  {
                    let byte_value = u8::from_str_radix(&value, 16);
                    match byte_value {
                        Ok(byte) => {
                            // println!("The byte value is: 0x{:X}", byte);
                            if file_content[pos] == byte 
                            {
                                // println!("Byte match!");
                                current_match.push((desc, format!("{}", byte)));
                            } else {
                                // println!("Not a match!");
                                is_valid = false;
                            }
                        }
                        Err(e) => {
                            println!("Failed to parse hex string: {}", e);
                            is_valid = false;
                        }
                    }
                    
                }
                
            },
            FieldType::Short => {
                // println!("Signature field is a short:");
                if value == 'x'.to_string() {
                    let in_file = &file_content[pos..pos+2];
                    if desc.trim() == "Header size," {
                        // Convert the slice into an array of 4 bytes
                        let size = in_file.try_into().expect("Failed to convert slice to array");

                        // Convert the bytes to a 32-bit unsigned integer (little-endian)
                        let size = u16::from_le_bytes(size);
                        // println!("Size in file: {}", size);

                        //Add it to the curretn signature match fields
                        current_match.push((desc, format!("{} bytes", size)));
                    } else if desc.trim() == "Length," {
                        // Convert the slice into an array of 4 bytes
                        let size = in_file.try_into().expect("Failed to convert slice to array");

                        // Convert the bytes to a 32-bit unsigned integer (little-endian)
                        let size_value: u16 = u16::from_le_bytes(size);
                        let final_size: u32 = size_value as u32 * 4;

                        // println!("Size in file: {}", final_size);

                        //Add it to the curretn signature match fields
                        current_match.push((desc, format!("{} bytes", final_size)));
                    } else {
                        // println!("Only for displaying purposes");
                        let in_file = &file_content[pos..pos+2];
                        let hex_string: String = in_file
                                .iter()
                                .map(|byte| format!("{:02X}", byte))  // Format each byte as two-digit hex
                                .collect::<Vec<String>>()
                                .join(" ");
                            current_match.push((desc, hex_string));
                    }
                }
            }
            FieldType::Long => {
                // println!("Signature field is a long:");
                if value == 'x'.to_string() {
                    // println!("Only for displaying purposes");
                    let in_file = &file_content[pos..pos+4];
                    if desc.trim() == "File size," {
                        // Convert the slice into an array of 4 bytes
                        let size = in_file.try_into().expect("Failed to convert slice to array");

                        // Convert the bytes to a 32-bit unsigned integer (little-endian)
                        let size = u32::from_le_bytes(size);
                        // println!("Size in file: {}", size);

                        // Check file size
                        let size_result = verify_size(&file_name, size as usize);
                        match size_result {
                            Ok(true) => {
                                //Add it to the curretn signature match fields
                                current_match.push((desc, format!("{} bytes", size)));
                            },
                            _ => {
                                is_valid =  false;

                            }
                        }
                    }      
                    else if desc.trim() == "Code size," {
                        // Convert the slice into an array of 4 bytes
                        let size = in_file.try_into().expect("Failed to convert slice to array");

                        // Convert the bytes to a 32-bit unsigned integer (little-endian) + 64 to account for the header size
                        let size = u32::from_le_bytes(size) + 64;
                        // println!("Size in file: {}", size);

                        // TODO: Check file size
                        let size_result = verify_size(&file_name, size as usize);
                        match size_result {
                            Ok(true) => {
                                //Add it to the curretn signature match fields
                                current_match.push((desc, format!("{} bytes", size)));
                            },
                            _ => {
                                is_valid =  false;

                            }
                        }

                        //Add it to the curretn signature match fields
                        //current_match.push((desc, format!("{} bytes", size)));
                    } else if desc.trim() == "Header size," {
                        // Convert the slice into an array of 4 bytes
                        let size = in_file.try_into().expect("Failed to convert slice to array");

                        // Convert the bytes to a 32-bit unsigned integer (little-endian)
                        let size = u32::from_be_bytes(size);
                        // println!("Size in file: {}", size);

                        // TODO: Check file size

                        //Add it to the curretn signature match fields
                        current_match.push((desc, format!("{} bytes", size)));
                    }
                    else {
                        let hex_string: String = in_file
                            .iter()
                            .map(|byte| format!("{:02X}", byte))  // Format each byte as two-digit hex
                            .collect::<Vec<String>>()
                            .join(" ");
                        current_match.push((desc, hex_string));
                    }
                } else {
                    let mut bytes = Vec::new();
    
                    // Iterate through the string two characters at a time
                    for i in (0..value.len()).step_by(2) {
                        // Get the two characters, convert them to a byte
                        let byte_str = &value[i..i+2];
                        match u8::from_str_radix(byte_str, 16) {
                            Ok(byte) => bytes.push(byte),
                            Err(e) => {
                                println!("Failed to parse '{}' as hex: {}", byte_str, e);
                                println!("{}", desc);
                                return;
                            }
                        }
                    }

                    // println!("Byte array: {:?}", bytes);

                    if file_content[pos..pos+4] != bytes[..] {
                        is_valid = false;
                    } else {
                        current_match.push((desc, String::from_utf8_lossy(&bytes).to_string()));
                    }
                }
                    
            }
            _ => {

            }
        }
    }
    // If the signature is still valid after going through all fields, lets add it to our result list.
    if is_valid {
        list_matches.push(current_match);
    }
}

// Read the file containing the signatures information to match them against the target firmware
fn read_signatures() -> Vec<Signature> {
    let mut results = Vec::new();

    let re = Regex::new(r"\s+").unwrap();

    if let Ok(lines) = read_lines("magic/vendors") {
        let mut fields_to_add: Vec<Field> = Vec::new();
        for line in lines.flatten() {

            let mut field = Field {
                ..Default::default()
            };

            // if line.as_str().starts_with("#") || line.as_str().starts_with("\n") || line.as_str().starts_with(" ") {
            if line.as_str().starts_with("#") || line.len() == 0 {
                // Ignore the lines that start with #
                if line.as_str().starts_with(" ") || line.as_str().starts_with("\n") || line.len() == 0 {
                    // println!("EOS LINE:\n{}", line);
                    if fields_to_add.len() > 0 {
                        let current_signature = Signature {
                            fields: fields_to_add.clone()
                        };
                        // println!("{:#?}", current_signature);
                        
                        results.push(current_signature);
                        fields_to_add.clear();
                    }

                } else {
                    // println!("ELSE: {}", line)
                }
                
            } else {
                // println!("GOOD LINE:\n{}", line);
                //println!("Line:\n{:#?}", line.as_bytes());

                let split = re.splitn(&line, 4);

                for (index, item) in split.enumerate() {
                    //println!("\n==========\nITEM: {:#?}\n==========\n", item.as_bytes());
                    // println!("\n==========\nITEM: {}\n==========\n", item);
                    match index {
                        0 => {
                            let mut num: usize  = 0;
                            if item.starts_with(">") {
                                let re = Regex::new(r">\s*(\d+)").unwrap();
                                if let Some(captures) = re.captures(item.trim()) {
                                    // Extract the number
                                    let number = &captures[1];
                                    // println!("Matched number: {}", number);
                                    num = number.trim().parse().expect("Not a valid number!");
                                    
                                } else {
                                    // println!("No match found");
                                }
                                
                            } else if item.is_empty() {
                                // println!("Empty item");
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

// Method to match the file size reported in the signature and the one calculated from the file
fn verify_size(file_path: &str, expected_size: usize) -> std::io::Result<bool> {
    let file = std::fs::read(file_path)?;
    if file.len() != expected_size {
        println!("file size doesnt match expected size");
        return Ok(false);
    } //if
    let checksum = X25.checksum(&file);
    println!("checksum: {:#X}", checksum);
    Ok(true)

} //verify_size

// Prints the data from the matched signatures
pub fn print_data(sig_matches: Vec<Vec<(String, String)>>) {
    println!("\n4. This is the information that you were looking for: ");
    println!("{:#?}", sig_matches);
}

// Helper method to read the lines of the signatures file
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_verify_size_no_file() {
        let result = verify_size("", 4);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_verify_size() {
        let result = verify_size("test_files/fw.bin", 4);
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_verify_size_false() {
        let result = verify_size("test_files/fw.bin", 154);
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_read_lines_no_file() {
        let result = read_lines("");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_read_lines() {
        let result = read_lines("test_files/fw.bin").unwrap();
        for line in result {
            assert_eq!(line.unwrap(), "aaaa");
        }
    }

    #[test]
    fn test_read_lines_false() {
        let result = read_lines("test_files/fw.bin").unwrap();
        for line in result {
            assert_ne!(line.unwrap(), "ferxxo");
        }
    }
}