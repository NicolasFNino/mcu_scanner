use std::fs::File;
use std::io::Read;

pub fn extract_file() -> [u8; 64]{

    let file_path: String = text_io::read!("{}");

    let mut file = File::open(file_path).unwrap();

    let mut file_content = [0u8; 64];

    // Reading the first 64 bytes of the file as bytes
    let result = file.read_exact(&mut file_content);

    // Quick exit if there was an error
    // TODO: Loop to try again after error or after completing the execution 
    if result.is_err() {
        println!("There was an error in the execution");
        return [0; 64]
    }

    // Printing the contents of the file. Only for debugging
    println!("\nThe contents of the file:");
    for byte in file_content {
        println!("{:#04X?}", byte)
    }
     
    println!("\n2. Extracting or decoding the contents of the input file:");

    // TODO: Actually check here if we need to convert from srec/intel_hex/zip/gz/etc... to bin, convert it and return the first 64/128 bytes

    file_content
}