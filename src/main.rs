use clap::{Arg, ArgMatches, Command};
use ethers::prelude::Abigen;
use eyre::Result;
use std::{fs, path::Path};

fn main() -> Result<()> {
    let matches = Command::new("generate")
        .about("Generates Rust file from ABI JSON")
        .arg(
            Arg::new("abi_source")
                .help("Sets the ABI JSON source file or directory")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("out_file")
                .help("Sets the output Rust file")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("contract_name")
                .help("Sets the contract name (not used for directory mode)")
                .required(false)
                .index(3),
        )
        .get_matches();

    generate_file(&matches)?;

    Ok(())
}

fn generate_file(matches: &ArgMatches) -> Result<()> {
    let abi_source = matches
        .get_one::<String>("abi_source")
        .expect("ABI source file or directory is required");

    let out_file = matches
        .get_one::<String>("out_file")
        .expect("Output file is required");

    let default = "".to_string();
    let contract_name = matches
        .get_one::<String>("contract_name")
        .unwrap_or(&default);

    if Path::new(abi_source).is_file() {
        generate_single_file(abi_source, out_file, contract_name)?;
        println!("File generated successfully!");
    } else if Path::new(abi_source).is_dir() {
        generate_files_from_directory(abi_source, out_file)?;
        println!("All files generated successfully!");
    } else {
        eprintln!("Error: The ABI source must be a file or a directory.");
        std::process::exit(1);
    }

    Ok(())
}

fn generate_single_file(abi_source: &str, out_file: &str, contract_name: &str) -> Result<()> {
    println!(
        "Generating Rust file for contract '{}' from ABI JSON: {}",
        contract_name, abi_source
    );

    let out_dir = Path::new(out_file);
    create_directory(out_dir)?;

    let output_file = format!("{}/{}.rs", out_file, contract_name);
    Abigen::new(contract_name, abi_source)?
        .generate()?
        .write_to_file(&output_file)?;

    Ok(())
}

fn generate_files_from_directory(directory: &str, out_file: &str) -> Result<()> {
    let entries = fs::read_dir(directory)?;

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "json" {
                        if let Some(file_name) = path.file_stem() {
                            if let Some(contract_name) = file_name.to_str() {
                                let contract_name = contract_name.to_string();
                                let abi_source = path.to_str().unwrap().to_string();
                                generate_single_file(&abi_source, &out_file, &contract_name)?;
                            }
                        }
                    }
                }
            } else if path.is_dir() {
                // If it's a directory, recursively call the function to handle files in subdirectories
                let dir_name = path.to_str().unwrap();
                generate_files_from_directory(dir_name, out_file)?;
            }
        }
    }

    Ok(())
}

fn create_directory(directory: &Path) -> Result<()> {
    if !directory.exists() {
        fs::create_dir_all(directory)?;
    }
    Ok(())
}
