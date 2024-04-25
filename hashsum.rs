use std::fs::File;
use std::io::{self, Read};
use std::io::stdin;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

fn main() {
    let mut file_path = String::new();
    let mut input_hashsum = String::new();

    println!("Enter your hashsum: ");

    match stdin().read_line(&mut input_hashsum) {
        Ok(bytes) => {
            println!("Ok. bytes entered = {}",
                     bytes
            );
        }
        Err(err) => {
            println!("Error: {}",
                     err
            )},
    }

    println!("Enter file path for checking hashsum: ");
    match stdin().read_line(&mut file_path) {
        Ok(bytes) => {
            println!("Ok. bytes entered = {}", bytes);
            file_path = file_path
                .trim()
                .to_string();
        },
        Err(err) => {
            println!("Error: {}",
                     err
            )},
    }

    match calculate_hash(&file_path) {
        Ok(hash) => {
            println!("Hashsum of {}: {}",
                     file_path,
                     hash
            );
            let check = check_hashsum(&input_hashsum
                .trim()
                .to_string(),
                 &hash
            );
            println!("Is hashsum of {} correct: {}",
                     file_path,
                     check
            );
        },
        Err(err) => {println!("Error: {}", 
                        err
        )},
    }

}

fn calculate_hash(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut hasher = Sha256::new();
    hasher.input(&buffer);

    Ok(hasher.result_str())
}

fn check_hashsum(input_hash: &String, output_hash: &String) -> bool {
    if *input_hash != *output_hash {
        return false;
    }
    return true;
}
