fn main() {
    // variables
    let mut x = 6;
    println!("The number is: {x}");
    x = 5;
    println!("The number is: {x}");

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("{}", THREE_HOURS_IN_SECONDS);

    // shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}")
    }
    println!("The value of y is: {y}")
}
