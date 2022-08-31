const DIGIT0: &str = " _ | ||_|";
const DIGIT1: &str = "     |  |";
const DIGIT2: &str = " _  _||_ ";
const DIGIT3: &str = " _  _| _|";
const DIGIT4: &str = "   |_|  |";
const DIGIT5: &str = " _ |_  _|";
const DIGIT6: &str = " _ |_ |_|";
const DIGIT7: &str = " _   |  |";
const DIGIT8: &str = " _ |_||_|";
const DIGIT9: &str = " _ |_| _|";
const DIGITS: &[&str] = &[
    DIGIT0, DIGIT1, DIGIT2, DIGIT3, DIGIT4, DIGIT5, DIGIT6, DIGIT7, DIGIT8, DIGIT9,
];

fn main() {
    //     _  _  _  _  _  _  _  _  _
    //    | || || || || || || || || |
    //    |_||_||_||_||_||_||_||_||_|

    //     |  |  |  |  |  |  |  |  |
    //     |  |  |  |  |  |  |  |  |

    //    _  _  _  _  _  _  _  _  _
    //    _| _| _| _| _| _| _| _| _|
    //   |_ |_ |_ |_ |_ |_ |_ |_ |_

    //    _  _  _  _  _  _  _  _  _
    //    _| _| _| _| _| _| _| _| _|
    //    _| _| _| _| _| _| _| _| _|

    //   |_||_||_||_||_||_||_||_||_|
    //     |  |  |  |  |  |  |  |  |

    //    _  _  _  _  _  _  _  _  _
    //   |_ |_ |_ |_ |_ |_ |_ |_ |_
    //    _| _| _| _| _| _| _| _| _|

    //    _  _  _  _  _  _  _  _  _
    //   |_ |_ |_ |_ |_ |_ |_ |_ |_
    //   |_||_||_||_||_||_||_||_||_|

    //    _  _  _  _  _  _  _  _  _
    //     |  |  |  |  |  |  |  |  |
    //     |  |  |  |  |  |  |  |  |

    //    _  _  _  _  _  _  _  _  _
    //   |_||_||_||_||_||_||_||_||_|
    //   |_||_||_||_||_||_||_||_||_|

    //    _  _  _  _  _  _  _  _  _
    //   |_||_||_||_||_||_||_||_||_|
    //    _| _| _| _| _| _| _| _| _|

    //       _  _     _  _  _  _  _
    //     | _| _||_||_ |_   ||_||_|
    //     ||_  _|  | _||_|  ||_| _|

    let first_line = "    _  _     _  _  _  _  _ ";
    let second_line = "  | _| _||_||_ |_   ||_||_|";
    let third_line = "  ||_  _|  | _||_|  ||_| _|";

    for i in 0..9 {
        let extracted_digit = extract_digit(first_line, second_line, third_line, i);
        let detected_digit = detect_digit(extracted_digit);
        print!("{}", detected_digit);
    }

    // for digit in DIGITS {
    //     print_digit(digit);
    // }
}

fn print_digit(digit: &str) {
    println!("{}", &digit[0..3]);
    println!("{}", &digit[3..6]);
    println!("{}", &digit[6..9]);
}

fn extract_digit(line1: &str, line2: &str, line3: &str, index: usize) -> String {
    let start: usize = index * 3;
    let stop = start + 3;

    format!(
        "{}{}{}",
        &line1[start..stop],
        &line2[start..stop],
        &line3[start..stop]
    )
}

fn detect_digit(digit_to_detect: String) -> u8 {
    let mut i = 0;
    for digit in DIGITS {
        if *digit == digit_to_detect {
            return i;
        }
        i += 1;
    }
    i
}
