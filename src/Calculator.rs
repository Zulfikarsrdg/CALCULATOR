
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}


fn calculate(op: Operation) -> f64 {

    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}


fn main() {

    println!("Enter the first number:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let num1: f64 = input.trim().parse().unwrap();


    println!("Enter the operation (+, -, *, /):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let op: char = input.trim().parse().unwrap();

    println!("Enter the second number:");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let num2: f64 = input.trim().parse().unwrap();


    let operation = match op {
        '+' => Operation::Add(num1, num2),
        '-' => Operation::Subtract(num1, num2),
        '*' => Operation::Multiply(num1, num2),
        '/' => Operation::Divide(num1, num2),
        _ => panic!("Invalid operation"),
    };


    let result = calculate(operation);


    println!("The result is: {}", result);
}
