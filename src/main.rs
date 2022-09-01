use std::collections::HashMap;

fn main() {
    let mut digits_hash = HashMap::new();

    populate_digits(&mut digits_hash);

    let first_line = "    _  _     _  _  _  _  _ ";
    let second_line = "  | _| _||_||_ |_   ||_||_|";
    let third_line = "  ||_  _|  | _||_|  ||_| _|";

    let detected_digits = (0..9)
        .map(|i| {
            detect_digit(
                &digits_hash,
                extract_digit(first_line, second_line, third_line, i),
            )
        })
        .collect::<Vec<u8>>();

    detected_digits.iter().for_each(|digit| print!("{}", digit));
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

    format!(
        "{}{}{}",
        &line1[start..stop],
        &line2[start..stop],
        &line3[start..stop]
    )
}

fn detect_digit(digits: &HashMap<&str, u32>, digit_to_detect: String) -> u8 {
    match digits.get(digit_to_detect.as_str()) {
        Some(index) => *index as u8,
        None => 255,
    }
}
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
