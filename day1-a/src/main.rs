use std::fs;
use std::str::FromStr;

fn main() {
    let file: String = fs::read_to_string("input-a.txt").unwrap();
    process(&file);
    println!("{:?}", process(&file))
}

fn process(input: &str) -> u32 {
    input.split('\n')
        .map(|line| -> u32 {
            let mut first = ' ';
            let mut second = ' ';
            for char in line.chars() {
                if char.is_digit(10) {
                    if first == ' ' {
                        first = char;
                    } else {
                        second = char
                    }
                }
            }
            if second == ' ' {
                second = first;
            }

            let mut string = String::from(first);
            string.push(second);
            u32::from_str(&string).unwrap()
        })
        .reduce(|acc, e| acc + e)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = process(input);
        assert_eq!(result, 142);
    }
}