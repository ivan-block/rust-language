pub fn run() {
    let person: (&str, &str, i8) = ("Ivan", "Nigeria", 24);

    // Using positions to print out the values
    println!("Hi, my name is {}, i am from {} and i am {}", person.0, person.1, person.2);

    // Using placeholders
    let (x, y, z) = person;
    println!("Hi my name is {x}, i am from {y} and i am {z}")
}