use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use std::str;

struct AccountNumber {
    number: String,
    line1: String,
    line2: String,
    line3: String,
}

impl AccountNumber {
    fn new(line1_str: &str, line2_str: &str, line3_str: &str) -> Self {
        AccountNumber {
            line1: line1_str.to_owned(),
            line2: line2_str.to_owned(),
            line3: line3_str.to_owned(),
            number: "".to_owned(),
        }
    }

    fn print(&self) {
        println!("{}", self.line1.trim_end());
        println!("{}", self.line2.trim_end());
        println!("{}", self.line3.trim_end());
    }
}

fn main() -> Result<(), Error> {
    let path = Path::new("resource\\accounts.txt");
    println!("{}", path.display());

    let mut buffered_file = match File::open(&path) {
        Ok(file) => BufReader::new(file),
        Err(why) => panic!("Could not read {}: {}", path.display(), why),
    };

    loop {
        match process_next_account_number(&mut buffered_file) {
            Ok(account_number) => println!("{}", account_number.number),
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
) -> Result<AccountNumber, Error> {
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

    let mut account_number = AccountNumber::new(&first_line, &second_line, &third_line);

    account_number.print();

    let detected_digits = (0..9)
        .map(|i| detect_digit(extract_digit(first_line, second_line, third_line, i)))
        .collect::<Vec<u8>>();

    let chars = detected_digits.iter().map(|d| d + 48).collect::<Vec<u8>>();

    let x = String::from_utf8(chars);
    account_number.number = x.unwrap();

    Ok(account_number)
}

fn extract_digit(line1: &str, line2: &str, line3: &str, index: usize) -> String {
    let start: usize = index * 3;
    let stop = start + 3;

    let composite_string = format!(
        "{}{}{}",
        &line1[start..stop],
        &line2[start..stop],
        &line3[start..stop]
    );

    composite_string
}

fn detect_digit(digit_to_detect: String) -> u8 {
    match digit_to_detect.as_str() {
        " _ | ||_|" => 0,
        "     |  |" => 1,
        " _  _||_ " => 2,
        " _  _| _|" => 3,
        "   |_|  |" => 4,
        " _ |_  _|" => 5,
        " _ |_ |_|" => 6,
        " _   |  |" => 7,
        " _ |_||_|" => 8,
        " _ |_| _|" => 9,
        _ => 255,
    }
}
