#[test]
fn day6() {
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    let file = File::open("inputs/6").unwrap();
    let buf = BufReader::new(file);
    let mut total = 0;
    let mut all: HashSet<char> = HashSet::new();
    let aa: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();
    for a in aa.iter() {
        if a == "" {
            total += all.len();
            all = HashSet::new();
            continue;
        }
        for c in a.chars() {
            all.insert(c);
        }
    }
    total += all.len();

    assert_eq!(total, 6549);
}

#[test]
fn day6_2() {
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::iter::FromIterator;

    let file = File::open("inputs/6").unwrap();
    let buf = BufReader::new(file);
    let mut total = 0;
    let mut all: HashSet<char> = HashSet::new();
    let aa: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();
    let mut none = false;
    for a in aa.iter() {
        if a == "" {
            none = false;
            println!("{}", all.len());
            total += all.len();
            all = HashSet::new();
            continue;
        }
        if none {
            continue;
        }
        if all.len() == 0 {
            all = HashSet::from_iter(a.chars());
        } else {
            let cur = HashSet::from_iter(a.chars());
            all = all.intersection(&cur).map(|x| *x).collect();
            if all.len() == 0 {
                none = true;
            }
        }
        println!("{:?}", all);
    }
    total += all.len();

    assert_eq!(total, 3466);
}
