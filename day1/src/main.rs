use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("You need to pass the part number in arg")
    }

    let x: (&str, fn(&str) -> u32) = match args[1].as_str() {
        "1" => ("input-a.txt", part_one),
        "2" => ("input-b.txt", part_two),
        _ => panic!("Not a valid part number"),
    };

    let file: String = fs::read_to_string(x.0).expect("The input file must exist");

    let start = std::time::Instant::now();
    let result = file.lines().map(x.1).sum::<u32>();
    eprintln!("{:?}", start.elapsed());

    println!("{:?}", result);
}

fn part_one(line: &str) -> u32 {
    let mut it = line.chars().filter_map(|char| char.to_digit(10));

    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("should be a valid number")
}

fn part_two(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            '1'
        } else if reduced_line.starts_with("two") {
            '2'
        } else if reduced_line.starts_with("three") {
            '3'
        } else if reduced_line.starts_with("four") {
            '4'
        } else if reduced_line.starts_with("five") {
            '5'
        } else if reduced_line.starts_with("six") {
            '6'
        } else if reduced_line.starts_with("seven") {
            '7'
        } else if reduced_line.starts_with("eight") {
            '8'
        } else if reduced_line.starts_with("nine") {
            '9'
        } else {
            reduced_line.chars().next().unwrap()
        };
        result.to_digit(10)
    });
    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("should be a valid number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part_one() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = input.lines().map(part_one).sum::<u32>();
        assert_eq!(result, 142);
    }

    #[test]
    fn test_process_part_two() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = input.lines().map(part_two).sum::<u32>();
        assert_eq!(result, 281);
    }
}
