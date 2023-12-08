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

fn main() {
    let input = get_input("input/input.txt");
    let games: Vec<Game> = parse_input(&input);
    println!("input is: {input}");
    // println!("found games {:#?}", games);
    let valid_games: Vec<&Game> = games.iter().filter(|game| game.is_valid()).collect();
    let sum_of_valid_game_numbers: usize = valid_games
        .iter()
        .map(|game| {
            print!("adding game {}", game.game_number);

            game.game_number
        })
        .sum();
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
        let game_number = line[index_of_column - 1..index_of_column]
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
        //        println!("SPLIT_STRING = {split_string}");
        split_string.parse::<usize>().unwrap()
    })
}
