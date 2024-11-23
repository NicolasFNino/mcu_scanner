///determines if contents represent binary blob - checks for specific file formats and returns false if content matches any of these:
///intel hex (:), motorola s record (s1, s2, s3), gzip (\x1F\x8B), or zip (pk)
///return true if none of them match and assume it is binary blob
pub fn is_binary_blob(contents: &[u8]) -> bool {
    //Intel Hex
    if contents.starts_with(b":") {
        return false;
    }//end if

    //Motorola SRecord
    if contents.starts_with(b"S1") || contents.starts_with(b"S2") || contents.starts_with(b"S3") {
        return false; 
    }//end if

    //Gzip
    if contents.starts_with(b"\x1F\x8B") {
        return false; 
    }//end if

    //Zip
    if contents.starts_with(b"PK") {
        return false; 
    }//end if

    //ToDo: Add other checks as needed

    // If none of the checks matched, we assume it's a binary blob
    true
}// end is_binary_blob