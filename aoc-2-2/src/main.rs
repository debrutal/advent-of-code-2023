#[derive(Debug)]
struct Game {
    draws: Vec<Draw>,
    game_number: usize,
}
impl Game {
    fn is_valid(&self) -> bool {
        let limit_reached = self
            .draws
            .iter()
            .find(|draw| draw.blue > MAX_BLUE || draw.red > MAX_RED || draw.green > MAX_GREEN);
        print!("filter game {:?}", self);
        limit_reached.is_none()
    }
    fn get_no_of_miminum_dies(&self) -> usize {
        let reds = self
            .draws
            .iter()
            .map(|draw| if draw.red == 0 { 1 } else { draw.red })
            .max()
            .unwrap();
        let blues = self
            .draws
            .iter()
            .map(|draw| if draw.blue == 0 { 1 } else { draw.blue })
            .max()
            .unwrap();
        let greens = self
            .draws
            .iter()
            .map(|draw| if draw.green == 0 { 1 } else { draw.green })
            .max()
            .unwrap();
        reds * blues * greens
    }
}
#[derive(Debug)]
struct Draw {
    red: usize,
    blue: usize,
    green: usize,
}
#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
}
const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;
const TEST_DATA: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

fn main() {
    let input = get_input("input/input.txt");
    let games: Vec<Game> = parse_input(&input);
    println!("input is: {input}");
    // println!("found games {:#?}", games);
    //let valid_games: Vec<&Game> = games.iter().filter(|game| game.is_valid()).collect();
    let sum_of_valid_game_numbers: usize =
        games.iter().map(|game| game.get_no_of_miminum_dies()).sum();
    println!("sum is {sum_of_valid_game_numbers}")
}

fn get_input(file: &str) -> String {
    std::fs::read_to_string(file).expect("cannot read file {file}")
}

fn parse_input(input: &str) -> Vec<Game> {
    println!("Parse Start ### ");
    let mut games = vec![];
    input.lines().for_each(|line| {
        let index_of_column = line
            .find(':')
            .unwrap_or_else(|| panic!("no column found for string {line}"));

        let game_number = line[..index_of_column]
            .split_whitespace()
            .last()
            .unwrap()
            .to_string()
            .parse::<usize>()
            .expect("cannot parse game_number");

        print!("game_number = {game_number} ");
        let draws_string = line[index_of_column..=line.len() - 1].split(';');

        let draws: Vec<Draw> = draws_string
            .map(|die_string| {
                let reds = find_amount_of_dies(die_string.to_string(), Color::Red);
                let blues = find_amount_of_dies(die_string.to_string(), Color::Blue);
                let greens = find_amount_of_dies(die_string.to_string(), Color::Green);
                Draw {
                    red: reds,
                    blue: blues,
                    green: greens,
                }
            })
            .collect();

        games.push(Game { draws, game_number });
    });
    games
}

fn find_amount_of_dies(string: String, color: Color) -> usize {
    match color {
        Color::Red => string.find("red"),
        Color::Blue => string.find("blue"),
        Color::Green => string.find("green"),
    }
    .map_or(0, |value| {
        //print!("VALUE = {value} ");
        let split_string = string[(value - 3)..value - 1]
            .split_whitespace()
            .last()
            .expect("This is not beautiful. fix me");
        //        print!("String is {string} ");
        //        print!("COLOR= {:?} ", color);
        // println!("SPLIT_STRING = {split_string}");
        split_string.parse::<usize>().unwrap()
    })
}
