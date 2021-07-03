pub mod hashing {
    use ring::digest;
    use ring::digest::Digest;
    use std::fs;

    pub fn string_hash(hash_type: String, value: Option<&str>) -> Result<Digest, String> {
        let value = value.unwrap().as_bytes();
        match hash_type.as_str() {
            "SHA1" => Ok(digest::digest(&digest::SHA1_FOR_LEGACY_USE_ONLY, value)),
            "SHA256" => Ok(digest::digest(&digest::SHA256, value)),
            "SHA512" => Ok(digest::digest(&digest::SHA512, value)),
            _ => Err(String::from("Error: Invalid hash type.")),
        }
    }

    pub fn file_hash(hash_type: String, value: Option<&str>) -> Result<Digest, String> {
        let contents = fs::read_to_string(value.unwrap());
        match contents {
            Ok(contents) => {
                match hash_type.as_str() {
                    "SHA1" => Ok(digest::digest(&digest::SHA1_FOR_LEGACY_USE_ONLY, contents.as_bytes())),
                    "SHA256" => Ok(digest::digest(&digest::SHA256, contents.as_bytes())),
                    "SHA512" => Ok(digest::digest(&digest::SHA512, contents.as_bytes())),
                    _ => Err(String::from("Error: Invalid hash type.")),
                }
            }
            _ => Err(String::from("Error: File not found.")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hashing;
    use data_encoding::HEXLOWER;

    #[test]
    fn check_string_hashing() {
        let string_value = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
        let hashed_value = hashing::string_hash("SHA256".to_string(), Some("hello"));
        let encoded_value = HEXLOWER.encode(hashed_value.unwrap().as_ref());
        assert_eq!(string_value, encoded_value, "Values are not equal!");
    }
    // add this
    // fn check_file_hashing() {
    //
    // }
}
