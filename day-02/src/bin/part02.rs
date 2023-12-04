use regex::Regex;

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
    input.lines().map(|x| process_game(x)).sum()
}

fn process_game(line: &str) -> u32 {
    let mut red_max = 0;
    let mut green_max = 0;
    let mut blue_max = 0;

    let games = line.split_once(":").unwrap().1;
    let cubes: Vec<CubesColor> = games.split(";").map(|game| game.split(",")).flatten().map(|cube_str| process_cube_str(cube_str)).collect();
    for cube in cubes{
        match cube.color {
            Color::Red => if red_max < cube.val { red_max = cube.val },
            Color::Green => if green_max < cube.val { green_max = cube.val },
            Color::Blue => if blue_max < cube.val { blue_max = cube.val },
        }
    }
    red_max * green_max * blue_max
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

