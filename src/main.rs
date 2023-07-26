use clap::{Arg, ArgMatches, Command};
use ethers::prelude::Abigen;
use eyre::Result;

fn generate_file(matches: &ArgMatches) -> Result<()> {
    let abi_source = matches
        .get_one::<String>("abi_source")
        .expect("ABI source file is required");

    let out_file = matches
        .get_one::<String>("out_file")
        .expect("Output file is required");

    let contract_name = matches
        .get_one::<String>("contract_name")
        .expect("Contract name is required");

    println!("Generating Rust file from ABI JSON: {}", abi_source);
    println!("Output file: {}", out_file);
    println!("Contract name: {}", contract_name);

    Abigen::new(contract_name, abi_source)?
        .generate()?
        .write_to_file(out_file)?;

    println!("File generation successful!");

    Ok(())
}

fn main() -> Result<()> {
    let matches = Command::new("generate")
        .about("Generates Rust file from ABI JSON")
        .arg(
            Arg::new("abi_source")
                .help("Sets the ABI JSON source file")
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
                .help("Sets the contract name")
                .required(true)
                .index(3),
        )
        .get_matches();

    generate_file(&matches)?;

    Ok(())
}
