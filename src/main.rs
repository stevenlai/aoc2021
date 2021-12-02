use std::fs;

fn main() {
    let contents = fs::read_to_string("day1.txt").unwrap();

    let contents = contents.as_str();
    println!("Day 1a: {:?}", day1a(contents));
    println!("Day 1b: {:?}", day1b(contents));
}

fn day1a(data: &str) -> usize {
    data.lines()
        .map(|d| d.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|d| d[1] > d[0])
        .count()
}

fn day1b(data: &str) -> usize {
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

#[test]
fn test_day1() {
    let input = "199
200
208
210
200
207
240
269
260
263";

    assert_eq!(day1a(input), 7);
    assert_eq!(day1b(input), 5);
}
