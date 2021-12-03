use std::str::FromStr;

fn main() {
    let contents = helpers::get_data_from_file("day1.txt");
    println!("Day 1a: {:?}", day1a(&contents));
    println!("Day 1b: {:?}", day1b(&contents));

    let contents = helpers::get_data_from_file("day2.txt");
    println!("Day2a: {:?}", day2a(&contents));
    println!("Day2b: {:?}", day2b(&contents));

    let contents = helpers::get_data_from_file("day3.txt");
    println!("Day 3a: {:?}", day3a(&contents))
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
    let (horizontal, depth) = data
        .lines()
        .map(|m| m.split(" ").collect::<Vec<&str>>())
        .fold((0, 0), |(horizontal, depth), m| {
            let val = m[1].parse::<u32>().unwrap();
            match Direction::from_str(m[0]).unwrap() {
                Direction::Forward => (horizontal + val, depth),
                Direction::Down => (horizontal, depth + val),
                Direction::Up => (horizontal, depth - val),
            }
        });

    horizontal * depth
}

fn day2b(data: &String) -> u32 {
    let (horizontal, depth, _) = data
        .lines()
        .map(|m| m.split(" ").collect::<Vec<&str>>())
        .fold((0, 0, 0), |(horizontal, depth, aim), m| {
            let val = m[1].parse::<u32>().unwrap();
            match Direction::from_str(m[0]).unwrap() {
                Direction::Forward => (
                    horizontal + m[1].parse::<u32>().unwrap(),
                    depth + (val * aim),
                    aim,
                ),
                Direction::Down => (horizontal, depth, aim + val),
                Direction::Up => (horizontal, depth, aim - val),
            }
        });

    horizontal * depth
}

fn day3a(data: &String) -> u32 {
    let grid: Vec<Vec<u8>> = data
        .lines()
        .map(|d| {
            d.trim()
                .split("")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect();

    let l = grid[0].len();
    let one_count = (0..l).fold(vec![], |mut acc, i| {
        acc.push(grid.iter().filter(|g| g[i] == 1).count());
        acc
    });

    let zero_count = (0..l).fold(vec![], |mut acc, i| {
        acc.push(grid.iter().filter(|g| g[i] == 0).count());
        acc
    });

    let gamma = one_count
        .iter()
        .zip(zero_count.iter())
        .map(|(o, z)| if o > z { "1" } else { "0" })
        .collect::<Vec<&str>>()
        .join("");

    let epsilon = one_count
        .iter()
        .zip(zero_count.iter())
        .map(|(o, z)| if o < z { "1" } else { "0" })
        .collect::<Vec<&str>>()
        .join("");

    let gamma = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();

    gamma * epsilon
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

#[test]
fn test_day3() {
    let input = String::from(
        "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
    );

    assert_eq!(day3a(&input), 198)
}
