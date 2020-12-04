#[derive(PartialEq, Debug)]
pub struct Entry {
    pub min: i64,
    pub max: i64,
    pub value: String,
    pub password: String,
}

impl Entry {
    pub fn new(line: String) -> Entry {
        let parts: Vec<String> = line.split(" ").map(|l| l.to_string()).collect();
        let mm: Vec<i64> = parts[0]
            .split("-")
            .map(|l| l.parse::<i64>().unwrap())
            .collect();
        let value: String = parts[1].trim_end_matches(":").to_string();

        Entry {
            min: mm[0],
            max: mm[1],
            value: value,
            password: parts[2].clone(),
        }
    }

    pub fn valid(&self) -> bool {
        let count: i64 = self.password.matches(&self.value).count() as i64;
        count >= self.min && count <= self.max
    }
    pub fn valid2(&self) -> bool {
        let first = self
            .password
            .chars()
            .nth(self.min as usize - 1)
            .unwrap()
            .to_string();
        let second = self
            .password
            .chars()
            .nth(self.max as usize - 1)
            .unwrap()
            .to_string();
        if first == self.value && second == self.value {
            false
        } else if first == self.value {
            true
        } else if second == self.value {
            true
        } else {
            false
        }
    }
}

#[test]
fn test_day2() {
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    let file = File::open("inputs/2").unwrap();
    let buf = BufReader::new(file);
    let mut total = 0;
    let mut total2 = 0;
    let _aa: Vec<Entry> = buf
        .lines()
        .map(|l| {
            let e = Entry::new(l.unwrap());
            if e.valid() {
                total += 1;
            }
            if e.valid2() {
                total2 += 1;
            }
            e
        })
        .collect();

    assert_eq!(total, 493);
    assert_eq!(total2, 593);
}
