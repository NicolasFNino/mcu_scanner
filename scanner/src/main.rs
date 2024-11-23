mod mods;

///entry point for application
///just welcomes user, lists available input files, extracts binary content from selected file (handling other formats), 
///matches content to signatures, prints extracted info
fn main() {
    println!("\nWelcome to MCU Scanner 🦀🦀🦀");

    mods::print::print_list_inputs();

    // Extract the binary blob from encoded file if needed (Intel hex, SREC)
    let (name, contents) = mods::extract::extract_file();
    
    if !contents.is_empty(){
        // Match signature and extract relevant information
        let sig_matches = mods::verify::verify_file(name, contents);

        // Print out the relevant information
        mods::verify::print_data(sig_matches);
    }
}
