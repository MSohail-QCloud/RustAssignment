use std::io;

fn main() {
    println!("Welcome to Rust Calculator");
    println!("Enter First Number");

    let mut totalmarks = String::new();

    io::stdin()
        .read_line(&mut totalmarks)
        .expect("Failed to get total marks");

    println!("Your total marks is {totalmarks}");
    println!("Please input your Number of Subjects.");

    let mut  numbofsubj = String::new();

    io::stdin()
        .read_line(&mut numbofsubj)
        .expect("Failed to get Number of Subjects");

    println!("Your subjects are {numbofsubj}");

    let marks_float: f64 = totalmarks.trim().parse().unwrap();
    let subjects_float: f64 = numbofsubj.trim().parse().unwrap();
    let average : f64= (marks_float/subjects_float)*100.0;

    println!("Your marks percentage is {average}")
}