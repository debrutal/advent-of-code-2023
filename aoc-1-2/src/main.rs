use std::fs::read_to_string;

const TEST_SAMPLE: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

const NUMBER_STRINGS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let path = "input/input.txt";
    let input = read_to_string(&path).expect(&format!("Could not read file {}", &path).to_string());

    get_numbers_and_sum(input);
    //    get_numbers_and_sum(TEST_SAMPLE.to_string());
}
fn get_numbers_and_sum(input: String) {
    let numbers: Vec<u32> = get_numbers(input);
    let sum: u32 = numbers.iter().sum();
    println!("sum is: {:}", sum);
}

fn get_numbers(input: String) -> Vec<u32> {
    let mut result = vec![];
    for line in input.lines() {
        result.push(get_first_and_last_number(line.to_string()));
    }
    result
}
// gets the first and last number in the given string
// also interprets written characters as number e.g. one, five, etc. up to nine
fn get_first_and_last_number(line: String) -> u32 {
    print!("{line} ");
    let result: String = match find_first(&line) {
        None => "0".to_string(),
        Some(first) => {
            let last = find_last(&line)
                .expect("This should not happen since find_first already found something!");
            let number_as_str: String = if first.0 == last.0 {
                println!("adding {}", first.0);
                format!("{}", first.0).to_string()
            } else {
                println!("adding {}{}", first.0, last.0);
                format!("{}{}", first.0, last.0).to_string()
            };

            number_as_str
        }
    };

    result
        .parse::<u32>()
        .expect(&format!("not a number {result}"))
}

fn find_first(line: &String) -> Option<(String, usize)> {
    let optional_number_index = line.chars().position(|e| e.is_digit(10));
    let mut result: Option<(String, usize)> = match optional_number_index {
        Some(number_index) => {
            let actual_number = line[number_index..=number_index].to_string();

            print!("fi_n = {number_index}; value = {actual_number} ");
            Some((actual_number, number_index))
        }
        None => None,
    };
    for word in NUMBER_STRINGS.iter() {
        if let Some(word_found_on_index) = line.find(word) {
            print!("fi_w = {word_found_on_index} ");
            result = match result {
                Some(inner_result) => {
                    if word_found_on_index < inner_result.1 {
                        let number = convert_word_to_number(word.to_string());
                        print!("; w_value = {number} ");
                        print!("## {word}");
                        Some((format!("{:}", number), word_found_on_index))
                    } else {
                        print!("AAAA: {}", inner_result.1);
                        Some(inner_result)
                    }
                }
                None => {
                    let number = convert_word_to_number(word.to_string());
                    Some((format!("{:}", number), word_found_on_index))
                }
            }
        }
    }
    result
}

fn find_last(line: &String) -> Option<(String, usize)> {
    let optional_number_index = line.chars().rev().position(|e| e.is_digit(10));

    let mut result: Option<(String, usize)> = match optional_number_index {
        None => None,
        Some(index) => {
            let number_index = line.len() - index as usize - 1;
            print!("li: {number_index} ");

            Some((line[number_index..=number_index].to_string(), number_index))
        }
    };

    for word in NUMBER_STRINGS.iter() {
        if let Some(word_found_on_index) = line.find(word) {
            result = match result {
                Some(inner_result) => {
                    if word_found_on_index > inner_result.1 {
                        let number = convert_word_to_number(word.to_string());
                        Some((format!("{:}", number), word_found_on_index))
                    } else {
                        Some(inner_result)
                    }
                }
                None => {
                    let number = convert_word_to_number(word.to_string());
                    Some((format!("{:}", number), word_found_on_index))
                }
            }
        }
    }
    //     for word in NUMBER_STRINGS.iter() {
    //         let indices: Vec<usize> = line.match_indices(word).map(|(i, _)| i).collect();
    //         if indices.len() > 0 {
    //             let word_found_on_index = indices[indices.len() - 1];
    //             if word_found_on_index > lowest_index && word_found_on_index > number_index {
    //                 lowest_index = word_found_on_index;
    //                 let number = convert_word_to_number(word.to_string());
    //                 result = format!("{:}", number);
    //             }
    //         }
    //     }
    result
}

fn convert_word_to_number(word: String) -> usize {
    print!("t ");
    NUMBER_STRINGS
        .iter()
        .position(|&item| item == word)
        .unwrap()
        + 1
}
