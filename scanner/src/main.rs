mod mods;

fn main() {
    println!("\nWelcome to MCU Scanner 🦀🦀🦀");
    
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
    let (name, contents) = mods::extract::extract_file();
    
    // Match signature and extract relevant information
    let sig_matches = mods::verify::verify_file(name, contents);

    // Print out the relevant information
    mods::verify::print_data(sig_matches);
}
