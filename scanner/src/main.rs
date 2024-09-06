use std::fs::File;
// use std::io::BufReader;
use std::io::Read;

// use text_io::read;

fn main() {
    println!("\nWelcome to MCU Scanner 🦀🦀🦀");
    
    println!("Please type the absolute path to your input file:");
    let file_path: String = text_io::read!("{}");

    let mut file = File::open(file_path).unwrap();

    let mut bytes = [0u8; 12];
    let result = file.read_exact(&mut bytes);

    if result.is_err() {
        println!("There was an error in the execution");
        return
    }

    println!("\nThe contents of the file:");
    for byte in bytes {
        println!("{:#04X?}", byte)
    }

    // let my_buf = BufReader::new(file.unwrap());
    // println!("This are the contents of the file:");
    // for byte_or_error in my_buf.bytes() {
    //     let byte: u8 = byte_or_error.unwrap();
    //     println!("{:b}", byte);
    // }
}
