use std::fs;

// //metadata
// pub fn print_metadata(metadata: &Metadata) {
//     println!("File Metadata:");
//     println!("Filen ame: {}", metadata.filename);
//     println!("File size: {} bytes", metadata.file_size);
//     println!("Entropy: {:.2}", metadata.entropy);
// }

// //file verif
// pub fn print_verification_result(result: bool) {
//     if result {
//         println!("Successful");
//     } else {
//         println!("Failed :()");
//     }
// }

pub fn print_list_inputs() {

    println!("\nSelect one of these files:");
    match fs::read_dir("/samples/") {
        Ok(paths) => {
            for p in paths {
                match p {
                    Ok(path) => {
                        println!("{}", path.path().display())
                    },
                    _ => { }
                }
            }
        }
        _ => { }
    };
}