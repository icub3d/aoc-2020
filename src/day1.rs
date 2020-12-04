pub fn day1(aa: &[i64]) -> i64 {
    for (i, a) in aa.iter().enumerate() {
        for b in aa[i..].iter() {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    0
}

pub fn day1_3(aa: &[i64]) -> i64 {
    for (i, a) in aa.iter().enumerate() {
        for (j, b) in aa[i..].iter().enumerate() {
            for c in aa[j..].iter() {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    0
}

#[test]
fn test_day1() {
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    let file = File::open("inputs/1").unwrap();
    let buf = BufReader::new(file);
    let aa: Vec<i64> = buf
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect();

    assert_eq!(day1(&aa), 987339);
}

#[test]
fn test_day1_3() {
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    let file = File::open("inputs/1").unwrap();
    let buf = BufReader::new(file);
    let aa: Vec<i64> = buf
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect();

    assert_eq!(day1_3(&aa), 259521570);
}
