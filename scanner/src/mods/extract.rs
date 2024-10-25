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
    file_content
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


fn calculate_file_entropy(file_path: &str) -> Result<f32, std::io::Error> {
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