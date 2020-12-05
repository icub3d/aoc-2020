pub fn solve(a: &str) -> usize {
    let mut rmin = 0;
    let mut rmax = 127;
    let mut r = 0;
    for x in 0..7 {
        let l = a.chars().nth(x).unwrap();
        if l == 'F' {
            rmax -= (rmax - rmin + 1) / 2;
            r = rmax;
        } else {
            rmin += (rmax - rmin + 1) / 2;
            r = rmin;
        }
    }
    let mut cmin = 0;
    let mut cmax = 7;
    let mut c = 0;
    for x in 7..10 {
        let l = a.chars().nth(x).unwrap();
        if l == 'L' {
            cmax -= (cmax - cmin + 1) / 2;
            c = cmax;
        } else {
            cmin += (cmax - cmin + 1) / 2;
            c = cmin;
        }
    }
    r * 8 + c
}

#[test]
fn test_day5() {
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    let file = File::open("inputs/5").unwrap();
    let buf = BufReader::new(file);
    let mut highest = 0;
    let mut all: Vec<usize> = Vec::new();
    let aa: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();
    for a in aa.iter() {
        let i = solve(a);
        all.push(i);
        if i > highest {
            highest = i;
        }
    }

    assert_eq!(highest, 933);

    let mut mine = 0;
    all.sort();
    for (i, x) in all.iter().enumerate() {
        if x + 1 != all[i + 1] {
            mine = x + 1;
            break;
        }
    }
    assert_eq!(mine, 711);
}
