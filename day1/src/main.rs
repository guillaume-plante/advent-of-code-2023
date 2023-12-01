use std::collections::HashMap;
use std::str::FromStr;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("You need to pass the part number in arg")
    }

    let x: (&str, fn(&str) -> u32) = match args[1].as_str() {
        "1" => ("input-a1.txt", part_one),
        "2" => ("input-b.txt", part_two),
        _ => panic!("Not a valid part number"),
    };

    let file: String = fs::read_to_string(x.0).expect("The input file must exist");

    let start = std::time::Instant::now();
    let result = process(&file, x.1);
    eprintln!("{:?}", start.elapsed());

    println!("{:?}", result);
}

fn process(input: &str, extractor: fn(&str) -> u32) -> u32 {
    input.split('\n').map(extractor).sum()
}

fn part_one(line: &str) -> u32 {
    let mut first = ' ';
    let mut second = ' ';

    for char in line.chars() {
        if !char.is_digit(10) {
            continue;
        }
        first = char;
        break;
    }
    for char in line.chars().rev() {
        if !char.is_digit(10) {
            continue;
        }
        second = char;
        break;
    }
    if second == ' ' {
        second = first;
    }

    let mut string = String::from(first);
    string.push(second);
    u32::from_str(&string).unwrap()
}

fn part_two(line: &str) -> u32 {
    let dict: HashMap<&str, char> = HashMap::from([
        ("zero", '0'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let mut first = ' ';
    let mut second = ' ';

    for (index, char) in line.chars().enumerate() {
        let result = string_digit(&dict, &line[index..]);

        if char.is_digit(10) {
            if first == ' ' {
                first = char;
            } else {
                second = char
            }
        } else if result != 'a' {
            if first == ' ' {
                first = result;
            } else {
                second = result
            }
        }
    }
    if second == ' ' {
        second = first;
    }

    let mut string = String::from(first);
    string.push(second);
    u32::from_str(&string).unwrap()
}

fn string_digit(dict: &HashMap<&str, char>, line: &str) -> char {
    for i in 0..line.len() {
        let y = &line[0..i + 1];
        let x = dict.get(y);
        if x.is_none() {
            continue;
        }
        return x.unwrap().clone();
    }
    'a'
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
        let result = process(input, part_one);
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
        let result = process(input, part_two);
        assert_eq!(result, 281);
    }
}
