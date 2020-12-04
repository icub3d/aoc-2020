pub fn slope(aa: &[String], right: usize, down: usize) -> usize {
    let mut trees = 0;
    let mut cur = 0;
    let mut skip = false;
    for a in aa[1..].iter() {
        if down == 2 {
            skip = !skip;
        }
        if skip {
            continue;
        }
        cur += right;
        if cur >= a.len() {
            cur = cur - a.len();
        }
        if a.chars().nth(cur).unwrap() == '#' {
            trees += 1;
        }
    }
    trees
}

#[test]
fn test_day3() {
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    let file = File::open("inputs/3").unwrap();
    let buf = BufReader::new(file);
    let aa: Vec<String> = buf.lines().map(|l| l.unwrap().to_string()).collect();

    let trees = slope(&aa, 3, 1);
    assert_eq!(trees, 164);

    let r1d1 = slope(&aa, 1, 1);
    let r3d1 = trees;
    let r5d1 = slope(&aa, 5, 1);
    let r7d1 = slope(&aa, 7, 1);
    let r1d2 = slope(&aa, 1, 2);
    assert_eq!(r1d1 * r3d1 * r5d1 * r7d1 * r1d2, 5007658656);
}
