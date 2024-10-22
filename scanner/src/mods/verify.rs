use std::{fs::File, io::{BufRead, BufReader, Lines, Result}, path::Path};

// TODO: The structures should be much more complex
//
// We need to have a criteria to 'discard' a signature based on each field
// examples: 1. file size is negative
//           2. a field value is not valid for the signature that is supposed to be a part of
//
extern crate crc;
const X25: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);


#[derive(Debug)]
pub struct Signature{
    fields: Vec<Field>,
}

#[derive(Debug)]
struct Field {
    position: u32,
    value_type: FieldType,
    constraint: String,
    description: String
}

#[derive(Debug)]
enum FieldType {
    Str(String),
    Date(String),
    Regex(String),
    Byte(u8),
    Short(u8, u8),
    Long(u8, u8, u8, u8),
}

pub fn verify_file(contents: Vec<u8>) -> Vec<Signature>{
    println!("\n3. Verifing the contents of the input file:");
    let mut signatures = read_signatures();

    let mut current_signature: Option<Signature> = None;

    let calc_crc = calculate_crc(&contents);
    let expect_crc: u16 = 0x906e;

    if calc_crc == expect_crc {
        println!("CRC check passed!");
    } else {
        println!("CRC check failed!");
        //ToD: handle this
    }

    current_signature = Some(Signature {
        fields: Vec::new()
    });

    signatures.push(current_signature.take().unwrap());

    signatures
}

fn read_signatures() -> Vec<Signature> {
    let mut results = Vec::new();

    if let Ok(lines) = read_lines("magic/vendors") {
        for line in lines.flatten() {
            if line.as_str().starts_with("#") || line.as_str().starts_with(" ") {
                // Ignore the lines that start with # or space
            } else {
                // Here is where the signature population should happen
                println!("{}", line)
            }
        }
    }

    results
}

fn calculate_crc(contents: &[u8]) -> u16 {
    X25.checksum(contents)
}

fn verify_with_size() -> bool {
    true
}

pub fn print_data(sig_matches: Vec<Signature>) {
    println!("\n4. This is the information that you are interested in: ");
    println!("{:#?}", sig_matches);
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}