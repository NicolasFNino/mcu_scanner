use std::fs::File;
use std::io::{self, Read, BufReader};
use std::num::ParseIntError;
extern crate bin_file;
extern crate entropy;
use shannon_entropy::shannon_entropy;
use flate2::read::GzDecoder;
use zip::ZipArchive;
use atty;

///extracts a firmware file from either user input or stdin
///if input is piped, reads content directly from stdin
///else, prompts the user for the file path and reads content
//allows recompression of .gz and .tar and converts other formats to binaries
//returns tuple with file path and content of bianry
pub fn extract_file() -> (String, Vec<u8>) {
    let mut file_path = String::new();
    let mut file_content = Vec::new();

    // check if input is from interactive input or piped
    let stdin_is_tty = atty::is(atty::Stream::Stdin);

    // maximum number of attempts to prevent infinite loop
    let max_attempts = 3;
    let mut attempts = 0;

    if !stdin_is_tty {
        // read the file content directly from stdin if piped
        io::stdin()
            .read_to_end(&mut file_content)
            .expect("Failed to read from stdin");
        
        if !file_content.is_empty() {
            println!("\nInput received from stdin.");
        } else {
            println!("Error: Failed to read input from stdin or input is empty.");
        }

        return ("<stdin>".to_string(), file_content);
    }

    loop {
        if stdin_is_tty {
            if attempts >= max_attempts {
                println!("Maximum attempts reached. Aborting...");
                break;
            }
            // ask for file path if in interactive mode
            println!("\n1. Please type the absolute path to your input file:");
            file_path = text_io::read!("{}");
        }

        // pipping will usually add quotes so this is to trim those off
        file_path = file_path.trim_matches('"').trim_matches('\'').to_string();

        // get the contents of the file and store it in the vector of u8 values
        file_content = read_firmware(file_path.as_str());

        // try again if nothing could be retrieved from the path provided by the user
        if file_content.is_empty() {
            attempts += 1;

            if stdin_is_tty {
                println!("Error - trying again!");
                if attempts >= max_attempts {
                    println!("Maximum attempts reached. Aborting...");
                    break;
                }
            } else {
                println!("Error: Failed to read file or file is empty.");
                if attempts >= max_attempts {
                    println!("Maximum attempts reached. Aborting...");
                    break;
                }

            }
        } else {
            if !is_encrypted(&file_path) {
                break;
            }
        }
    }

    // Printing the contents of the file. Only for debugging
    // println!("\nThe contents of the file:");
    // for byte in &file_content {
    //     println!("{:#04X?}", byte)
    // }

    println!("\n2. Extracting or decoding the contents of the input file:");

    if !file_content.is_empty() {

        if file_content[0] == b':' {
            println!("intel->binary");
            file_content = intel_hex_to_binary(&file_content).unwrap_or(Vec::new());
        } else if file_content[0] == b'S' {
            println!("SREC -> binary");
            file_content = srec_to_binary(&file_content).unwrap_or(Vec::new());
        } else if file_path.ends_with(".gz") {
            println!("GZ -> binary");
            file_content = decompress_gz(file_path.as_str()).unwrap_or(Vec::new());
        } else if file_path.ends_with(".zip") {
            println!("ZIP -> binary");
            file_content = extract_zip(file_path.as_str()).unwrap_or(Vec::new());
        } else {
            println!("unknown format, assuming binary");
        }

        // get the first 256 bytes from the file for performance 
        file_content.truncate(256);

    } else {
        println!("Error: File content is empty. Unable to process the file.");
    }

    return (file_path, file_content)
    
}

///decode a hex file to binary representation
///prameters: string slice of one line of intel hex data
//returns vector of binary data or error if there is something wrong/failure during parsing
fn intel_hex_to_binary(data: &[u8]) -> Result<Vec<u8>, ParseIntError> {
    let data_str = String::from_utf8_lossy(data);
    let mut binary_data = Vec::new();
    for line in data_str.lines() {
        if line.starts_with(':') {
            let bytes = parse_hex_data(line)?;
            binary_data.extend(bytes);
        }
    }
    Ok(binary_data)
}

///helper method for the intel_hex_to_binary method
///prameters: string slice of one line of intel hex data
//returns vector of binary data or error if there is something wrong/failure during parsing
fn parse_hex_data(datahex: &str) -> Result<Vec<u8>, ParseIntError> {
    let mut binary_data = Vec::new();
    let datahex_len = u8::from_str_radix(&datahex[1..3], 16)?;
    let address = u16::from_str_radix(&datahex[3..7], 16)?;
    let datahex_type = u8::from_str_radix(&datahex[7..9], 16)?;
    let mut data_idx = 9;
    if datahex_type == 0x00 {
        for _ in 0..datahex_len {
            let byte = u8::from_str_radix(&datahex[data_idx..data_idx + 2], 16)?;
            binary_data.push(byte);
            data_idx += 2;
        }
    }
    Ok(binary_data)
}

/// decode Motorola SRecord fole to binary representation
///paramters: slice of bytes representing the S-Record data
///returns vector of binary data or error if there is something wrong/failure during parsing
fn srec_to_binary(data: &[u8]) -> Result<Vec<u8>, ParseIntError> {
    let data_str = String::from_utf8_lossy(data);
    let mut binary_data = Vec::new();
    for line in data_str.lines() {
        if line.starts_with('S') {
            let bytes = parse_srec_data(line)?;
            binary_data.extend(bytes);
        }
    }
    Ok(binary_data)
}

///helper method for the srec_to_binary method
///parameters: string slice of a single line of S-Record data
///returns vector of binary data or error if there is something wrong/failure during parsing
fn parse_srec_data(datasrec: &str) -> Result<Vec<u8>, ParseIntError> {
    let mut binary_data = Vec::new();
    let datasrec_type = &datasrec[1..2];
    let count = u8::from_str_radix(&datasrec[2..4], 16)?;
    let mut data_idx = 4;
    if datasrec_type == "1" || datasrec_type == "2" || datasrec_type == "3" {
        let address_len = match datasrec_type {
            "1" => 4, 
            "2" => 6,
            "3" => 8,  
            _ => 0,
        };
        data_idx += address_len;  
        while data_idx < (4 + count as usize * 2) - 2 {
            let byte = u8::from_str_radix(&datasrec[data_idx..data_idx + 2], 16)?;
            binary_data.push(byte);
            data_idx += 2;
        }
    }
    Ok(binary_data)
}

///decompresses gz file
///parameters: path to gx file
///returns vecror of binary fata or error if decompression fails
fn decompress_gz(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    let file = File::open(file_path)?;
    let mut gz = GzDecoder::new(file);
    let mut binary_data = Vec::new();
    gz.read_to_end(&mut binary_data)?;
    Ok(binary_data)
}

// decompresses zip file
///parameter: path to the zip file
///returns vector of binary data or error if failure
fn extract_zip(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    let file = File::open(file_path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut binary_data = Vec::new();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        file.read_to_end(&mut binary_data)?;
    }
    Ok(binary_data)
}


///converts hexadecimal string into binary
///parameter: hex string
///returns vector of binary data or error if failure
fn hex_str_to_binary(hex: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
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


///reads hex file and converts to binary
///parameter: path to file with hex data
///returns vector of vinary data or error if failure during parsing
fn hex_file_to_binary(file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
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


///calculates entropy of file contents
///parameter: path to the file
///returns entropy as float or error if failure while reading file
fn calculate_file_entropy(file_path: &str) -> Result<f32, std::io::Error> {
    let file = File::open(file_path)?; 
    let mut buf_reader = BufReader::new(file); 
    let mut buffer = Vec::new(); 
    buf_reader.read_to_end(&mut buffer)?;
    let contentstr = String::from_utf8_lossy(&buffer);
    let entropy: f32 = shannon_entropy(&contentstr); 
    Ok(entropy)
}


///check to see if file encrypted based on entropy
///parameters: path to file
///rreturns true if file entropy = encryptions and false if not
fn is_encrypted(file_path: &str) -> bool {
    let entropy = calculate_file_entropy(&file_path);

    match entropy {
        Ok(numero) => {
            if numero > 9.5 {
                println!("This file is encrypted, entropy: {}\nAborting...", numero);
                return true;
            }
        },
        _ => {
            println!("There was an error trying to calculate the entropy.\nAborting...");
                return true;
        }
    }
    return false;
}

///helper method to get the file contents and return it as a Vector of u8 values
///reads content into vector of bytes
///param: path to firmware
///returns vector of bytes with file content or empty vector if failure
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

//////unit tests
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_read_firmware_no_file() {
        let result = read_firmware("");
        assert_eq!(result.is_empty(), true);
    }

    #[test]
    fn test_read_firmware() {
        let result = read_firmware("test_files/fw.bin");
        assert_eq!(result, vec![97,97,97,97]);
    }

    #[test]
    fn test_is_encrypted() {
        let result = is_encrypted("test_files/fw.bin");
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_encrypted_no_file() {
        let result = is_encrypted("");
        assert_eq!(result, true);
    }

    #[test]
    fn test_calculate_entropy() {
        let result = calculate_file_entropy("test_files/fw.bin");
        assert_eq!(result.unwrap(), 0f32);
    }

    #[test]
    fn test_calculate_entropy_no_file() {
        let result = calculate_file_entropy("");
        assert_eq!(result.is_err(), true);
    }

}