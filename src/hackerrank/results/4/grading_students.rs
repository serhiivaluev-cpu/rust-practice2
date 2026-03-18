use std::io::{self, BufRead};

fn grading_students(grade: i32) -> i32 {
    if grade < 38 {
        grade
    } else {
        let next5 = ((grade / 5) + 1) * 5;
        if next5 - grade < 3 {
            next5
        } else {
            grade
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..n {
        let grade: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        let result = grading_students(grade);
        println!("{}", result);
    }
}
