use std::fs::File;
use std::io::{Read, BufReader};
use std::collections::HashMap;
use std::num::ParseIntError;
extern crate bin_file;
extern crate entropy;


pub fn extract_file() -> [u8; 64]{
    

    let mut file_content = [0u8; 64];

    loop {
        println!("\n1. Please type the absolute path to your input file:");

        let file_path: String = text_io::read!("{}");

        let mut file = File::open(file_path).unwrap();

        // Reading the first 64 bytes of the file as bytes
        let result = file.read_exact(&mut file_content);
    
        if result.is_err() {
            println!("error - trying again!");
            continue;
        } //if
        break;
    } //loop
    
    
    
    // Quick exit if there was an error
    // TODO: Loop to try again after error or after completing the execution 
   //if result.is_err() {
       // println!("There was an error in the execution");
       //return [0; 64]
    //}

    // Printing the contents of the file. Only for debugging
    println!("\nThe contents of the file:");
    for byte in file_content {
        println!("{:#04X?}", byte)
    }
     
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
pub fn read_firmware<'a>(file_path: &'a str) {
    //try to open with file path
    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);
        //loop through each line in the file
        //try to read line - include error checking if read fails
        //.contains to see if it has "" for specific thing we are looking for?
        //use else if^
    } else {
        //print error for trying to open the file
    } //if (open file)
} //read_firmware