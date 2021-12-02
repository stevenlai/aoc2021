use std::str::FromStr;

fn main() {
    let contents = helpers::get_data_from_file("day1.txt");
    println!("Day 1a: {:?}", day1a(&contents));
    println!("Day 1b: {:?}", day1b(&contents));

    let contents = helpers::get_data_from_file("day2.txt");
    println!("Day2: {:?}", day2a(&contents));
    println!("Day2: {:?}", day2b(&contents));
}

mod helpers {
    pub fn get_data_from_file(path: &str) -> String {
        std::fs::read_to_string(path).unwrap()
    }
}

fn day1a(data: &String) -> usize {
    data.lines()
        .map(|d| d.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|d| d[1] > d[0])
        .count()
}

fn day1b(data: &String) -> usize {
    data.lines()
        .map(|d| d.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .windows(3)
        .map(|d| d.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|d| d[1] > d[0])
        .count()
}

enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

fn day2a(data: &String) -> u32 {
    let depths = data
        .lines()
        .map(|m| m.split(" ").collect::<Vec<&str>>());

    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;

    for m in depths {
        let val: u32 = m[1].parse::<u32>().unwrap();
        match Direction::from_str(m[0]).unwrap() {
            Direction::Forward => horizontal = horizontal + val,
            Direction::Down => depth = depth + val,
            Direction::Up => depth = depth - val,
        }
    }

    horizontal * depth
}

fn day2b(data: &String) -> u32 {
    let depths = data
        .lines()
        .map(|m| m.split(" ").collect::<Vec<&str>>());

    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;

    for m in depths {
        let val: u32 = m[1].parse::<u32>().unwrap();
        match Direction::from_str(m[0]).unwrap() {
            Direction::Forward => {
                horizontal = horizontal + val;
                depth = depth + (val * aim);
            }
            Direction::Down => aim = aim + val,
            Direction::Up => aim = aim - val,
        }
    }

    horizontal * depth
}

#[test]
fn test_day1() {
    let input = String::from(
        "199
200
208
210
200
207
240
269
260
263",
    );

    assert_eq!(day1a(&input), 7);
    assert_eq!(day1b(&input), 5);
}

#[test]
fn test_day2() {
    let input = String::from(
        "forward 5
down 5
forward 8
up 3
down 8
forward 2",
    );

    assert_eq!(day2a(&input), 150);
    assert_eq!(day2b(&input), 900);
}
