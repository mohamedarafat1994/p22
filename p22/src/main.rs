use p22::calculator;

fn main() {
    println!("Add: {}", calculator::celsius2farenheit(30));
    println!("Subtract: {}", calculator::farenheit2celsius(86));
    println!("Multiply: {}", calculator::fibonacci_loop(3));
    println!("Multiply: {}", calculator::fibonacci_rec(3));
}
