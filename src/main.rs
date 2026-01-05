use hashit::*;
use std::env;
use std::path::Path;

pub fn hashit(flag: &str, argument: &str) -> Result<String, HashitError> {
    match flag {
        "-p" => {
            let input = argument.to_owned();
            let input_string = InputString::new(input);
            input_string.hash_sha256()
        }
        _ => {
            let input = Path::new(argument);
            let input_file = InputFile::new(input);
            input_file.hash_sha256()
        }
    }
}

fn main() -> Result<(), HashitError> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => println!("Please supply flags: -p to hash a password, -f to hash a file"),
        2 => println!("Format ./hashitbin -p password or ./hashitbin -f /filepath/file.extension"),
        3 => {
            let flag = &args[1];
            let input = &args[2];
            let hash = hashit(flag, input)?;
            println!("SHA256 hash: {}", hash);
        }
        _ => println!(
            "Too many arguments. Format ./hashitbin -p password or ./hashitbin -f /filepath/file.extension"
        ),
    }

    Ok(())
}
