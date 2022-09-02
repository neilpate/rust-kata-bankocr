use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::{collections::HashMap, path::Path};

fn main() -> Result<(), Error> {
    let mut digits_hash = HashMap::new();

    populate_digits(&mut digits_hash);

    let path = Path::new("resource\\accounts.txt");
    println!("{}", path.display());

    let mut buffered_file = match File::open(&path) {
        Ok(file) => BufReader::new(file),
        Err(why) => panic!("Could not read {}: {}", path.display(), why),
    };

    let mut line1 = String::new();
    let mut line2 = String::new();
    let mut line3 = String::new();

    buffered_file.read_line(&mut line1)?;
    let first_line = &line1.as_str();

    buffered_file.read_line(&mut line2)?;
    let second_line = &line2.as_str();

    buffered_file.read_line(&mut line3)?;
    let third_line = &line3.as_str();

    let detected_digits = (0..9)
        .map(|i| {
            detect_digit(
                &digits_hash,
                extract_digit(first_line, second_line, third_line, i),
            )
        })
        .collect::<Vec<u8>>();

    detected_digits.iter().for_each(|digit| print!("{}", digit));

    Ok(())
}

fn populate_digits(digits: &mut HashMap<&str, u32>) {
    digits.insert(" _ | ||_|", 0);
    digits.insert("     |  |", 1);
    digits.insert(" _  _||_ ", 2);
    digits.insert(" _  _| _|", 3);
    digits.insert("   |_|  |", 4);
    digits.insert(" _ |_  _|", 5);
    digits.insert(" _ |_ |_|", 6);
    digits.insert(" _   |  |", 7);
    digits.insert(" _ |_||_|", 8);
    digits.insert(" _ |_| _|", 9);
}

fn _print_digit(digit: &str) {
    println!("{}", &digit[0..3]);
    println!("{}", &digit[3..6]);
    println!("{}", &digit[6..9]);
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

fn detect_digit(digits: &HashMap<&str, u32>, digit_to_detect: String) -> u8 {
    match digits.get(digit_to_detect.as_str()) {
        Some(index) => *index as u8,
        None => 255,
    }
}
