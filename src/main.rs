//use acc;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

mod account_manager;

fn main() -> Result<(), Error> {
    let path = Path::new("resource\\accounts.txt");
    println!("{}", path.display());

    let mut buffered_file = match File::open(&path) {
        Ok(file) => BufReader::new(file),
        Err(why) => panic!("Could not read {}: {}", path.display(), why),
    };

    loop {
        match process_next_account_number(&mut buffered_file) {
            Ok(account_number) => {
                println!("{}", account_number);
                // println!("Debug: {:#?}", account_number);    // testing the pretty printed debug implementation
            }
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
        println!();
    }

    Ok(())
}

fn process_next_account_number(
    buffered_file: &mut BufReader<File>,
) -> Result<account_manager::AccountNumber, Error> {
    let mut line1 = String::new();
    let mut line2 = String::new();
    let mut line3 = String::new();
    buffered_file.read_line(&mut line1)?;
    let first_line = &line1.as_str();
    buffered_file.read_line(&mut line2)?;
    let second_line = &line2.as_str();
    let c = buffered_file.read_line(&mut line3)?;
    let third_line = &line3.as_str();

    if c == 0 {
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "End of file detected.",
        ));
    }

    let account_number =
        account_manager::AccountNumber::new(&first_line, &second_line, &third_line);

    Ok(account_number)
}
