#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! serde_json = "1.0.128"
//! bothan-core = "0.0.1-alpha.4"
//! ```

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use bothan_core::registry::{Invalid, Registry};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("unexpected arguments: {:?}", args);
        std::process::exit(1);
    }

    let path = Path::new(args[1].as_str());
    let registry: Registry<Invalid> = {
        let file = File::open(path).expect("Failed to open registry file");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).expect("Failed to parse registry file")
    };
    registry.validate().expect("Registry is invalid");

    println!("Registry is valid");
}
