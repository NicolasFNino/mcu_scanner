use std::fs::File;
use std::io::Read;

mod mods;

fn main() {
    println!("\nWelcome to MCU Scanner 🦀🦀🦀");
    
    // Selecting an input file. 
    // TODO: Better way to do it? 
    println!("Please type the absolute path to your input file:");
    let file_path: String = text_io::read!("{}");

    let mut file = File::open(file_path).unwrap();

    let mut file_content = [0u8; 64];

    // Reading the first 64 bytes of the file as bytes
    let result = file.read_exact(&mut file_content);

    // Quick exit if there was an error
    // TODO: Loop to try again after error or after completing the execution 
    if result.is_err() {
        println!("There was an error in the execution");
        return
    }

    // Printing the contents of the file. Only for debugging
    println!("\nThe contents of the file:");
    for byte in file_content {
        println!("{:#04X?}", byte)
    }

    // TODO: 
    // 1. Check if image is a binary blob
    //    Not in Intel Hex format, Motorola SRecord, or compressed.
    // 2. Decode/decompress the file to turn it into simple binary if needed
    //    decode_decompress(file_contents or file)

    // TODO: Match the contents of the file to one of the signatures.
    //       Still trying to figure out what would be the best way to store and match the signatured
    //       a. keep the libmagic structure and extend existing libmagic parsers to extract metadata
    //       b. get creativew and do something else 

    // Extract/decompress if needed
    mods::extract::extract_file(file);
    
    // Match signature and extract relevant information
    mods::verify::verify_file(file_content);

    // Print out the relevant information
    mods::print::print_data();

}
