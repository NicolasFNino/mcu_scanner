use mods::extract::extract_file;
use mods::verify::{verify_file, Signature};
use mods::print::{print_firmware_contents, print_signature_data};

fn main() {
    let firmware_content = extract_file();
    print_firmware_contents(&firmware_content);

    let signatures: Vec<Signature> = verify_file(firmware_content);
    print_signature_data(&signatures);
}
