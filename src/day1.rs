use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
pub fn run_part1() -> io::Result<()> {
    let fname = "day1_input.txt";
    let f = File::open(fname)?;
    let f = BufReader::new(f);
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut nlines = 0;
    for line in f.lines() {
        let l = line.unwrap();
        let mut it = l.split_whitespace();
        let l = it.next().unwrap().parse::<i32>().unwrap();
        let r = it.next().unwrap().parse::<i32>().unwrap();
        left.push(l);
        right.push(r);
        nlines += 1;
    }
    left.sort();
    right.sort();
    let mut total = 0;
    for i in 0..nlines {
        let l = left[i];
        let r = right[i];
        total += (r - l).abs();
    }
    println!("Part 1 Total: {}", total);
    Ok(())
}
pub fn run_part2() -> io::Result<()> {
    let fname = "day1_input.txt";
    let f = File::open(fname)?;
    let f = BufReader::new(f);
    let mut left: Vec<i32> = Vec::new();
    let mut right: BTreeMap<i32, i32> = BTreeMap::new();
    for line in f.lines() {
        let l = line.unwrap();
        let mut it = l.split_whitespace();
        let l = it.next().unwrap().parse::<i32>().unwrap();
        let r = it.next().unwrap().parse::<i32>().unwrap();
        left.push(l);
        *right.entry(r).or_default() += 1;
    }
    let mut total = 0;
    for n in left.iter() {
        let count = right.get(n).unwrap_or(&0);
        total += n * count;
    }
    println!("Part 2 Total: {}", total);
    Ok(())
}
