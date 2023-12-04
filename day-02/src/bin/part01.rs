use regex::Regex;

const RED_LIMIT: u32 = 12;
const GREEN_LIMIT: u32 = 13;
const BLUE_LIMIT: u32 = 14;

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct CubesColor {
    color: Color,
    val: u32
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", games_power(input));
}

fn games_power(input: &str) -> u32 {
    input.lines().map(process_game).sum()
}

fn process_game(line: &str) -> u32 {
    let num_re = Regex::new(r"(\d)+").unwrap();
    let game_num = num_re.find(line).unwrap().as_str().parse::<u32>().unwrap_or(0);

    let games = line.split_once(':').unwrap().1;
    let valid_game = games.split(';').flat_map(|game| game.split(',')).map(process_cube_str).all(check_cube);
    if valid_game {
        return game_num
    }
    0
}

fn process_cube_str(cube_str: &str) -> CubesColor {
    let num_re = Regex::new(r"(\d)+").unwrap();
    let color_re = Regex::new(r"[a-z]+").unwrap();

    let cube_num = num_re.find(cube_str).unwrap().as_str().parse::<u32>().unwrap_or(0);
    let cube_color = color_re.find(cube_str).unwrap().as_str();

    match cube_color {
        "red" => CubesColor{ color: Color::Red, val: cube_num },
        "green" => CubesColor{ color: Color::Green, val: cube_num },
        "blue" => CubesColor{ color: Color::Blue, val: cube_num },
        x => {println!("Invalid color {}", x);
            CubesColor{ color: Color::Blue, val: 0 }
        }
    }
}

fn check_cube(cube: CubesColor) -> bool {
    match cube.color {
        Color::Red => cube.val <= RED_LIMIT,
        Color::Green => cube.val <= GREEN_LIMIT,
        Color::Blue => cube.val <= BLUE_LIMIT
    }
}

