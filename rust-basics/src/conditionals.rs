pub fn run() {
    let number = 3;
    if number < 10 {
        println!("Condition is true")
    } else if number % 3 == 0 {
        println!("Condition is true and same number")
    } else {
        println!("Condition is false!");
    }
}