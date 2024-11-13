mod mods;

fn main() {
    println!("\nWelcome to MCU Scanner 🦀🦀🦀");

    // Extract the binary blob from encoded file if needed (Intel hex, SREC)
    let (name, contents) = mods::extract::extract_file();
    
    // Match signature and extract relevant information
    let sig_matches = mods::verify::verify_file(name, contents);

    // Print out the relevant information
    mods::verify::print_data(sig_matches);
}
