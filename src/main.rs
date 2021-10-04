use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush()
        .expect("Failed to flush");
    stdin().read_line(input)
        .expect("Failed to read");
}

fn main() {
    println!("Welcome to the nigga's calculator");
    println!("----------------");

    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();

    println!("What is the first number");
    read(&mut num1);

    println!("What is the second number");
    read(&mut num2);

    println!("What operation would you like to do [+-*/]");
    read(&mut operator);

    let num1: f32 = num1.trim().parse().unwrap();
    let num2: f32 = num2.trim().parse().unwrap();
    let operator: char = operator.trim().chars().next().unwrap();

    let operators = String::from("+-*/");

    if !operators.contains(operator) {
        println!("Unkown Operator");
        return;
    }

    let result = match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => panic!("Error in operator")
    };
    println!("The result of {} {} {} = {}", num1, num2, operator, result);
}
