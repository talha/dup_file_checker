use std::fs;
use std::fs::File;
use std::collections::{HashMap};
use crc::{Crc, CRC_32_ISCSI};
use std::io::prelude::*;
use clap::Parser;

/// Program to find duplicate files in a given directory.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Directory path to find duplicate files in
    #[clap(short, long, value_parser, default_value = ".")]
    directory: String,

    /// Show all files
    #[clap(short, long, value_parser)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    let default_directory = args.directory;    
    
    println!("Default directory: {}", default_directory);

    // Filename, Hash
    let mut file_info: HashMap<String, String> = HashMap::new();

    for entry in fs::read_dir(default_directory).unwrap() {
        if let Ok(entry) = entry {
            //let metadata = entry.metadata().unwrap();
            let file_type = entry.file_type().unwrap();
            if file_type.is_file() == true {
                let crc_hash = Crc::<u32>::new(&CRC_32_ISCSI);
                let mut digest = crc_hash.digest();
                
                // get file contents as bytes
                let mut file = File::open(entry.path()).unwrap();
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).unwrap();
                
                // calculate crc32 hash
                digest.update(&buffer);
                let crc_hash = digest.finalize();
                let crc_hash = format!("{:x}", crc_hash);
                
                // add to hashmap
                file_info.insert(entry.path().to_string_lossy().to_string(), crc_hash);
            }
        }
    }
    // Find duplicate values in file_info HashMap
    let mut duplicate_values: HashMap<String, String> = HashMap::new();
    for (filename, crc_hash) in file_info.iter() {
        if file_info.values().filter(|&x| x == crc_hash).count() > 1 {
            duplicate_values.insert(filename.to_string(), crc_hash.to_string());
        }
    }

    if args.verbose == true {
        println!("All files: {:#?}", file_info);
    }
    println!("Duplicate files: {:#?}", duplicate_values);
}
