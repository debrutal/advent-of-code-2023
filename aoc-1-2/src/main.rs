use std::fs::read_to_string;

const NUMBER_STRINGS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let path = "input/input.txt";
    let input = read_to_string(&path).expect(&format!("Could not read file {}", &path).to_string());
    //println!("{input}");

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
    print!("text {line}");
    let first = find_first(&line);
    let last = find_last(&line);
    let number = format!("{}{}", first, last);
    println!("adding {first}{last}");
    number
        .parse::<u32>()
        .expect(&format!("not a number {number}").to_string())
}

fn find_first(line: &String) -> String {
    let number_index = line.chars().position(|e| e.is_digit(10)).unwrap() as usize;
    print!("fi = {number_index} ");
    let mut result = line[number_index..number_index + 1].to_string();
    let mut lowest_index = usize::MAX;
    for word in NUMBER_STRINGS.iter() {
        if let Some(word_found_on_index) = line.find(word) {
            if word_found_on_index < lowest_index && word_found_on_index < number_index {
                lowest_index = word_found_on_index;
                let number = convert_word_to_number(word.to_string());
                result = format!("{:}", number);
            }
        }
    }
    result
}

fn find_last(line: &String) -> String {
    let number_index =
        line.len() - line.chars().rev().position(|e| e.is_digit(10)).unwrap() as usize - 1;
    print!("li: {number_index} ");

    let mut result = line[number_index..=number_index].to_string();

    let mut lowest_index = usize::MIN;
    for word in NUMBER_STRINGS.iter() {
        let indices: Vec<usize> = line.match_indices(word).map(|(i, _)| i).collect();
        if indices.len() > 0 {
            let word_found_on_index = indices[indices.len() - 1];
            if word_found_on_index > lowest_index && word_found_on_index > number_index {
                lowest_index = word_found_on_index;
                let number = convert_word_to_number(word.to_string());
                result = format!("{:}", number);
            }
        }
    }
    result
}
fn convert_word_to_number(word: String) -> usize {
    print!("t ");
    NUMBER_STRINGS
        .iter()
        .position(|&item| item == word)
        .unwrap()
}
