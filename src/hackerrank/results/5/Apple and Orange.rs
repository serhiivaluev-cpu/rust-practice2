use std::io;

pub fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: &[i32],
    oranges: &[i32],
) -> (i32, i32) {
    let apple_count = apples
        .iter()
        .map(|&d| a + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    let orange_count = oranges
        .iter()
        .map(|&d| b + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    (apple_count, orange_count)
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let st: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let s = st[0];
    let t = st[1];

    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let ab: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let a = ab[0];
    let b = ab[1];

    input.clear();

    io::stdin().read_line(&mut input).unwrap();

    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let apples: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let oranges: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let (apple_count, orange_count) =
        count_apples_and_oranges(s, t, a, b, &apples, &oranges);

    println!("{}", apple_count);
    println!("{}", orange_count);
}
