fn main() {

    another_function(5);
    print_label_measurement(5, 'h');

    let y = {
        let x = 5;
        x + 1
    };

    println!("The value of y is: {y}");

    let z = five();

    println!("The value of z is: {z}");

    let b = six(5);
    println!("The value of b is: {b}")
}

fn another_function(x: u32) {
    println!("The value of x is: {x}");
}

fn print_label_measurement(value: i32, label: char) {
    println!("The measurement is: {value}{label}")
}

// Function with Return Type
fn five() -> i32 {
    5
}

fn six(a: i32) -> i32 {
    a + 1
}
