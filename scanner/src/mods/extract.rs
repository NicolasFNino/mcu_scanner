use std::fs::File;
use std::io::{Read, BufReader};
use std::collections::HashMap;
use std::num::ParseIntError;
extern crate bin_file;
extern crate entropy;
use shannon_entropy::shannon_entropy; 


pub fn extract_file() -> Vec<u8>{
    

    let mut file_content = Vec::new();

    loop {
        println!("\n1. Please type the absolute path to your input file:");

        let file_path: String = text_io::read!("{}");

        file_content = read_firmware(file_path.as_str());
    
        if file_content.is_empty() {
            println!("error - trying again!");
            continue;
        }
        break;
    }

    // Printing the contents of the file. Only for debugging
    // println!("\nThe contents of the file:");
    // for byte in &file_content {
    //     println!("{:#04X?}", byte)
    // }
     
    println!("\n2. Extracting or decoding the contents of the input file:");

    // TODO: Actually check here if we need to convert from srec/intel_hex/zip/gz/etc... to bin, convert it and return the first 64/128 bytes
    if file_content[0] == b':' {
        println!("intel->binary");
        
    }else if file_content[0] == b'S' {
        println!("SREC -> binary");
    }else{
        println!("unknwon so assumed binary");
    } //add other checks to this for zip and gz and anything else that needs to be done
    file_content.truncate(64);
    file_content
}

fn intel_hex_to_binary(data: &[u8]) -> Result<Vec<u8>, ParseIntError> {
    let data_str = String::from_utf8_lossy(data);
    let mut binary_data = Vec::new();

    for line in data_str.lines() {
        if line.starts_with(':') {
            let bytes = parse_hex_record(line)?;
            binary_data.extend(bytes);
        }
    }

    Ok(binary_data)
}

fn parse_hex_record(record: &str) -> Result<Vec<u8>, ParseIntError> {
    let mut binary_data = Vec::new();
    let record_len = u8::from_str_radix(&record[1..3], 16)?;
    let address = u16::from_str_radix(&record[3..7], 16)?;
    let record_type = u8::from_str_radix(&record[7..9], 16)?;
    let mut data_idx = 9;
    if record_type == 0x00 {
        for _ in 0..record_len {
            let byte = u8::from_str_radix(&record[data_idx..data_idx + 2], 16)?;
            binary_data.push(byte);
            data_idx += 2;
        }
    }
    Ok(binary_data)
}

fn srec_to_binary(data: &[u8]) -> Result<Vec<u8>, ParseIntError> {
    let data_str = String::from_utf8_lossy(data);
    let mut binary_data = Vec::new();
    for line in data_str.lines() {
        if line.starts_with('S') {
            let bytes = parse_srec_record(line)?;
            binary_data.extend(bytes);
        }
    }
    Ok(binary_data)
}

fn parse_srec_record(record: &str) -> Result<Vec<u8>, ParseIntError> {
    let mut binary_data = Vec::new();
    let record_type = &record[1..2];
    let count = u8::from_str_radix(&record[2..4], 16)?;
    let mut data_idx = 4;
    if record_type == "1" || record_type == "2" || record_type == "3" {
        let address_len = match record_type {
            "1" => 4, 
            "2" => 6,
            "3" => 8,  
            _ => 0,
        };
        data_idx += address_len;  
        while data_idx < (4 + count as usize * 2) - 2 {
            let byte = u8::from_str_radix(&record[data_idx..data_idx + 2], 16)?;
            binary_data.push(byte);
            data_idx += 2;
        }
    }
    Ok(binary_data)
}


pub fn hex_str_to_binary(hex: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    let hex = hex.trim_start_matches("0x");

    let hex = if hex.len() % 2 != 0 {
        format!("0{}", hex)
    } else {
        hex.to_string()
    };//end if else

    let mut binary_vec = Vec::new();

    for i in (0..hex.len()).step_by(2) {
        let byte = u8::from_str_radix(&hex[i..i + 2], 16)?;
        binary_vec.push(byte);
    }//end for
    Ok(binary_vec)
}//end hex_str_to_binary

pub fn hex_file_to_binary(file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?; 
    let mut hex_data = String::new();
    file.read_to_string(&mut hex_data)?;
    let hex_data = hex_data.trim_start_matches("0x");

    let hex_data = if hex_data.len() % 2 != 0 {
        format!("0{}", hex_data)
    } else {
        hex_data.to_string()
    };//end if else

    let mut binary_vec = Vec::new();

    for i in (0..hex_data.len()).step_by(2) {
        let byte = u8::from_str_radix(&hex_data[i..i + 2], 16)?;
        binary_vec.push(byte);
    }//end for
    Ok(binary_vec)
}//end hex_file_to_binary


pub fn calculate_file_entropy(file_path: &str) -> Result<f32, std::io::Error> {
    let file = File::open(file_path)?; 
    let mut buf_reader = BufReader::new(file); 
    let mut buffer = Vec::new(); 
    buf_reader.read_to_end(&mut buffer)?;
    let contentstr = String::from_utf8_lossy(&buffer);
    let entropy: f32 = shannon_entropy(&contentstr); 
    Ok(entropy)
}



//idk if this signature is correct - not sure if lifetime is correct/needed
pub fn read_firmware<'a>(file_path: &'a str) -> Vec<u8> {
    //try to open with file path
    match std::fs::read(file_path) {
        Ok(file) =>  {
            return file;
        }
        Err(err) =>{
            println!("{}", err);
            return Vec::new();
        } 
    }
}


