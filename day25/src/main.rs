// puzzle input:
// 1965712
// 19072108

use mod_exp::mod_exp;

fn transform(subject_number: usize, loop_size: usize) -> usize {
    mod_exp(subject_number, loop_size, 20201227)
    // This is actually a modular exponentiation after some research.
    // let mut value = 1;
    // for _ in 0..loop_size {
    //     value = value * subject_number;
    //     value = value % 20201227;
    // }
    // value
}

fn main() {
    // println!("transform({}, {}): {}", 7, 8, transform(7, 8));
    // println!("transform({}, {}): {}", 7, 11, transform(7, 11));
    // println!("transform({}, {}): {}", 7, 8, transform(17807724, 8));
    // println!("transform({}, {}): {}", 7, 11, transform(5764801, 11));

    // for x in 0..1000 {
    //     println!("transform({}, {}) = {}", 7, x, transform(7, x));
    // }
    //println!("transform({}, {}) = {}", 7, first_loop_size, t);

    // Figure out he two loop sizes.
    let mut first_loop_size = 1;
    loop {
        let t = transform(7, first_loop_size);
        if t == 1965712 {
            break;
        }
        first_loop_size += 1;
    }
    println!("{}", first_loop_size);

    let mut second_loop_size = 1;
    loop {
        let t = transform(7, second_loop_size);
        if t == 19072108 {
            break;
        }
        second_loop_size += 1;
    }
    println!("{}", second_loop_size);

    println!("encryption key: {}", transform(1965712, second_loop_size));
}
