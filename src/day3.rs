use regex::Regex;
use std::fs::read_to_string;
use std::io::Result;

fn parse_muls(input: &str) -> i32 {
    let p = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    p.captures_iter(input)
        .map(|res| res.extract())
        .map(|(_, [l, r])| l.parse::<i32>().unwrap() * r.parse::<i32>().unwrap())
        .sum()
}

pub fn run_part1() -> Result<()> {
    let s = read_to_string("day3_input.txt")?;
    println!("Part 1: {}", parse_muls(&s));
    Ok(())
}

fn parse_muls_dos_donts(input: &str) -> i32 {
    let p = Regex::new(r"((?<do>do\(\))|(?<dont>don't\(\))|(mul\((?<mul_l>\d+),(?<mul_r>\d+)\)))")
        .unwrap();
    let mut enabled = true;
    let mut total: i32 = 0;
    for cap in p.captures_iter(input) {
        if cap.name("dont").is_some() {
            enabled = false;
        } else if cap.name("do").is_some() {
            enabled = true;
        } else if enabled {
            let l = cap.name("mul_l").unwrap().as_str().parse::<i32>().unwrap();
            let r = cap.name("mul_r").unwrap().as_str().parse::<i32>().unwrap();
            total += l * r;
        }
    }
    total
}

pub fn run_part2() -> Result<()> {
    let s = read_to_string("day3_input.txt")?;
    println!("Part 2: {}", parse_muls_dos_donts(&s));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_muls() {
        let s = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(parse_muls(s), 161)
    }
    #[test]
    fn test_parse_muls_dos_donts() {
        let s = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(parse_muls_dos_donts(s), 48)
    }
}
