use fancy_regex::Regex;
use std::fs;

pub fn solve() {
    solve_second();
}

fn solve_first() {
    if let Ok(contents) = fs::read_to_string("src/inputs/1") {
        let result: u32 = contents
            .lines()
            .map(|line| {
                line.chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect::<Vec<u32>>()
            })
            .map(|numbers| numbers.first().unwrap() * 10 + numbers.last().unwrap())
            .fold(0, |acc, x| acc + x);
        println!("Result: {}", result);
        // Use the lines vector here
    } else {
        println!("Failed to read the file");
    }
    println!("one!");
}

// maps the string of a single digit number, both spelled out and as a digit, to a u32
fn map_digit(digit: &str) -> u32 {
    match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        _ => panic!("invalid digit"),
    }
}

fn line_to_str_vec<'a>(line: &'a str, re: &Regex) -> Vec<&'a str> {
    let captures: Vec<&str> = re
        .captures_iter(line)
        .flat_map(|capture| {
            capture
                .unwrap()
                .iter()
                .map(|x| x.unwrap().as_str())
                .collect::<Vec<&str>>()
        })
        // filter out empty strings
        .filter(|x| !x.is_empty())
        .collect();
    captures
}

fn solve_second() {
    println!("second!");
    let re = Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine|0|1|2|3|4|5|6|7|8|9))")
        .unwrap();
    let sample_line = "abcone2threexyz";
    let c = re.captures_iter(sample_line);
    print!("captures: \n");
    for capture in c {
        println!("{:?}", capture.unwrap().iter().map(|x| x.unwrap().as_str()));
    }

    if let Ok(contents) = fs::read_to_string("src/inputs/1_1") {
        let result: u32 = contents
            .lines()
            .map(|line| {
                let captures: Vec<&str> = line_to_str_vec(line, &re);
                // gets the first and last captures, which are the two numbers

                if let (Some(first), Some(last)) = (captures.first(), captures.last()) {
                    let first = map_digit(first);
                    let last = map_digit(last);
                    println!("{}", line);
                    println!("{} {}", first, last);
                    return first * 10 + last;
                    // Use `first` and `last` here.
                }
                println!("failed to parse line: ");
                println!("{}", line);
                0
            })
            .fold(0, |acc, x| acc + x);
        println!("Result: {}", result);
        // Use the lines vector here
    } else {
        println!("Failed to read the file");
    }
    println!("one!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use fancy_regex::Regex;

    #[test]
    fn test_line_to_str_vec() {
        let re =
            Regex::new(r"(?:one|two|three|four|five|six|seven|eight|nine|0|1|2|3|4|5|6|7|8|9)")
                .unwrap();
        let line = "one two three 4 5 six";
        let expected = vec!["one", "two", "three", "4", "5", "six"];
        assert_eq!(line_to_str_vec(line, &re), expected);
    }

    #[test]
    fn test_overlaps() {
        let re =
            Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine|0|1|2|3|4|5|6|7|8|9))")
                .unwrap();
        let line = "sevenine";
        let expected = vec!["seven", "nine"];
        assert_eq!(line_to_str_vec(line, &re), expected);
    }

    #[test]
    fn test_line_to_str_vec_no_match() {
        let re =
            Regex::new(r"(?:one|two|three|four|five|six|seven|eight|nine|0|1|2|3|4|5|6|7|8|9)")
                .unwrap();
        let line = "hello world";
        let expected: Vec<&str> = Vec::new();
        assert_eq!(line_to_str_vec(line, &re), expected);
    }
}
