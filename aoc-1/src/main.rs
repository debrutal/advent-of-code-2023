use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
    let path = "input/input1.txt";
    let input = read_to_string(&path).expect(&format!("Could not read file {}", &path).to_string());
    //println!("{input}");

    let numbers: Vec<u32> = get_numbers(input);
    let sum: u32 = numbers.iter().sum();
    println!("sum is: {:}", sum);
}

fn get_numbers(input: String) -> Vec<u32> {
    let mut result = vec![];
    for line in input.lines() {
        result.push(get_first_and_second(line.to_string()));
    }
    result
}

fn get_first_and_second(line: String) -> u32 {
    let numbers: Vec<char> = line.chars().filter(|e| e.is_digit(10)).collect();
    let number = format!("{}{}",numbers.first().unwrap().to_owned(), numbers.last().unwrap().to_owned());
    number.parse::<u32>().expect(&format!("not a number {number}").to_string())

}
