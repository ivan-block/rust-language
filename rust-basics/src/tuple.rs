pub fn run() {
    let person: (&str, &str, i8) = ("Ivan", "Nigeria", 24);

    println!("Hi, my name is {}, i am from {} and i am {}", person.0, person.1, person.2)
}