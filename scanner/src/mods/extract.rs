use std::fs::File;
use std::io::{Read, BufReader};
use std::collections::HashMap;
use std::num::ParseIntError;
extern crate bin_file;
extern crate entropy;


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

fn hex_to_binary(hex: &str) -> Result<String, std::num::ParseIntError> {
    let number = u64::from_str_radix(hex, 16)?; // Parse hex string into a u64
    Ok(format!("{:b}", number)) // Convert number to binary string

}

fn calculate_entropy(file_path: &str) -> Result<f64, std::io::Error> {
    let file = File::open(file_path)?;
    
    let mut entropy = 0.0;

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