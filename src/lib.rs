pub mod day1 {
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
}
pub mod day2 {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    fn _as(n: i32) -> (i32, i32) {
        (n.abs(), n.signum())
    }
    fn report_safe(report: Vec<i32>) -> bool {
        let mut wins = report.windows(2);
        let mut sign: i32 = 0;
        while let Some(&[fst, snd]) = wins.next() {
            let res = fst - snd;
            let (abs, sgn) = _as(res);
            if sign == 0 {
                sign = sgn
            } else if sign != sgn {
                return false;
            }
            if !(1..=3).contains(&abs) {
                return false;
            }
        }
        true
    }
    pub fn run_part1() -> io::Result<()> {
        let fname = "day2_input.txt";
        let f = File::open(fname)?;
        let f = BufReader::new(f);
        let mut total_safe = 0;
        for line in f.lines() {
            let l = line.unwrap();
            let l = l
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            if report_safe(l) {
                total_safe += 1;
            }
        }
        println!("Part 1 Total Safe: {}", total_safe);
        Ok(())
    }
    fn ixs(n: usize) -> Vec<Vec<usize>> {
        let all = (0..n).collect::<Vec<usize>>();
        let mut out = vec![all];
        for skip in 0..n {
            let mut start: Vec<usize> = (0..skip).collect();
            let mut rest = (skip + 1..n).collect();
            start.append(&mut rest);
            out.push(start);
        }
        out
    }
    #[cfg(test)]
    #[test]
    fn test_ixs() {
        let expected_3 = vec![vec![0, 1, 2], vec![1, 2], vec![0, 2], vec![0, 1]];
        assert_eq!(ixs(3), expected_3);
        let expected_4 = vec![
            vec![0, 1, 2, 3],
            vec![1, 2, 3],
            vec![0, 2, 3],
            vec![0, 1, 3],
            vec![0, 1, 2],
        ];
        assert_eq!(ixs(4), expected_4);
        let expected_5 = vec![
            vec![0, 1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![0, 2, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 4],
            vec![0, 1, 2, 3],
        ];
        assert_eq!(ixs(5), expected_5);
    }
    fn pluck_ixs(report: Vec<i32>, ixs: Vec<usize>) -> Vec<i32> {
        ixs.iter().map(|&ix| report[ix]).collect()
    }
    fn report_safe2(report: Vec<i32>) -> bool {
        for v_ixs in ixs(report.len()).into_iter() {
            let plucked = pluck_ixs(report.clone(), v_ixs);
            if report_safe(plucked) {
                return true;
            }
        }
        false
    }
    pub fn run_part2() -> io::Result<()> {
        let fname = "day2_input.txt";
        let f = File::open(fname)?;
        let f = BufReader::new(f);
        let mut total_safe = 0;
        for line in f.lines() {
            let l = line.unwrap();
            let l = l
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            if report_safe2(l) {
                total_safe += 1;
            }
        }
        println!("Part 2 Total Safe: {}", total_safe);
        Ok(())
    }
}
