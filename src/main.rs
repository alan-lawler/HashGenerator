extern crate clap;
use clap::{Arg, App};
use hash_generator::hashing;

fn main() {
    let matches = App::new("Hash Generator Coded in Rust")
        .version("1.0")
        .author("Alan L. <github.com/alan-lawler>")
        .about("SHA1, SHA256, and SHA512 Hash Generator")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("FILE")
            .help("File to hash")
            .takes_value(true)
        )
        .arg(Arg::with_name("string")
            .short("s")
            .long("string")
            .value_name("STRING")
            .help("String to hash")
        )
        .arg(Arg::with_name("hash")
            .short("H")
            .long("hash")
            .value_name("HASH")
            .help("Hashes: SHA1, SHA256, SHA512")
            .required(true)
            .case_insensitive(true)
        )
        .get_matches();

    let hash_type = matches.value_of("hash").unwrap().to_uppercase();

    if matches.is_present("file") && matches.is_present("string") {
        println!("Error: Cannot use -s and -f together.");
    } else if matches.is_present("file") {
        match hashing::file_hash(hash_type, matches.value_of("file")) {
            Ok(result) => println!("{:?}", result),
            Err(e) => println!("{}", e)
        }
    } else if matches.is_present("string") {
        match hashing::string_hash(hash_type, matches.value_of("string")) {
            Ok(result) => println!("{:?}", result),
            Err(e) => println!("{}", e)
        }
    }
}
