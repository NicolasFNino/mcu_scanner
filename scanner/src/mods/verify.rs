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
    magic_number: String,
    vendor: String,
    architecture: String,
    series: String,
    fields: Vec<Field>,
}

#[derive(Debug)]
struct Field {
    file_size: u32,
    version: String,
    base_address: u32,
    file_offset: u32,
    entry_point: u32
}

pub fn verify_file(contents: [u8; 64]) -> Vec<Signature>{
    println!("\n3. Verifing the contents of the input file:");
    let mut signatures = Vec::new();

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
        magic_number: String::new(),
        vendor: String::new(),
        architecture: String::new(),
        series: String::new(),
        fields: Vec::new()
    });

    signatures.push(current_signature.take().unwrap());

    signatures
}

pub fn calculate_crc(contents: &[u8]) -> u16 {
    X25.checksum(contents)
}

fn verify_with_size() -> bool {
    true
}

pub fn print_data(sig_matches: Vec<Signature>) {
    println!("\n4. This is the information that you are interested in: ");
    println!("{:#?}", sig_matches);
}