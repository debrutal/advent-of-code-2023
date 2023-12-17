fn main() {
    let input = std::fs::read_to_string("input/input.txt").expect("failed to read file");
    let map = init_map(&input);
    let special_characters = map.find_special_characters();
    let neighbours_numbers = map.get_neighbours(special_characters);
    println!("Found: {:?}", neighbours_numbers);
}

enum Sign {
    Dot,
    Number,
    Special,
}

struct Number {
    value: usize,
    location: Vec<(usize, Vec<usize>)>,
}
struct EngineMap {
    map: Vec<Vec<char>>,
}

impl EngineMap {
    fn find_special_characters(&self) -> Vec<(usize, usize)> {
        let mut result = vec![];

        for (y, row) in self.map.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                match value {
                    '#' | '$' | '+' | '*' => result.push((x, y)),
                    _ => {}
                };
            }
        }
        result
    }
    fn get_numbers(&self) {
        let mut result = vec![];

        let map_iterator = self.map.iter();
        for (y, row) in map_iterator.enumerate() {
            let mut numbers_in_row = vec![];
            let row_iterator = row.iter();
            for (x, character) in row_iterator.enumerate() {
                if character.is_digit(10) {
                    numbers_in_row.push(character.to_digit(10).unwrap());
                    if let Some(peek) = row_iterator.peekable().peek() {
                        peek.is_digit(10);
                        // expand number explorsation
                    }
                }
            }
        }
    }
    fn get_neighbours(&self, special_characters: Vec<(usize, usize)>) -> Vec<usize> {
        for character in special_characters {
            vec![2]
        }
    }
}

fn init_map(string: &String) -> EngineMap {
    let mut engine_map = vec![];
    for line in string.lines() {
        let characters = line.chars().collect::<Vec<_>>();
        engine_map.push(characters);
    }
    EngineMap { map: engine_map }
}
