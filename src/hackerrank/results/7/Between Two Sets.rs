use std::io;

fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    fn gcd(x: i32, y: i32) -> i32 {
        if y == 0 { x } else { gcd(y, x % y) }
    }

    fn lcm(x: i32, y: i32) -> i32 {
        x * y / gcd(x, y)
    }

    let lcm_a = a.iter().copied().reduce(|x, y| lcm(x, y)).unwrap();
    let gcd_b = b.iter().copied().reduce(|x, y| gcd(x, y)).unwrap();

    let mut count = 0;
    let mut multiple = lcm_a;

    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // n m
    let _nm: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    input.clear();
    io::stdin().read_line(&mut input).unwrap(); // a
    let a: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    input.clear();
    io::stdin().read_line(&mut input).unwrap(); // b
    let b: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let total = get_total_x(&a, &b);
    println!("{}", total);
}
