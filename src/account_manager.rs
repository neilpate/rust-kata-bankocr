use std::fmt;

#[derive(Debug)]
pub struct AccountNumber {
    pub number: String,
    line1: String,
    line2: String,
    line3: String,
}

impl AccountNumber {
    pub fn new(line1_str: &str, line2_str: &str, line3_str: &str) -> Self {
        let mut an = AccountNumber {
            line1: line1_str.to_owned(),
            line2: line2_str.to_owned(),
            line3: line3_str.to_owned(),
            number: "".to_owned(),
        };

        let detected_digits = (0..9)
            .map(|i| detect_digit(extract_digit(line1_str, line2_str, line3_str, i)))
            .collect::<Vec<u8>>();

        let chars = detected_digits.iter().map(|d| d + 48).collect::<Vec<u8>>();

        let x = String::from_utf8(chars);
        an.number = x.unwrap();

        an
    }

    pub fn print(&self) {
        println!("{}", self.line1.trim_end());
        println!("{}", self.line2.trim_end());
        println!("{}", self.line3.trim_end());
        println!("{}", self.number);
    }
}

impl fmt::Display for AccountNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.number.as_str())
    }
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
