pub fn run() {
    // Print to console
    println!("This is the print from the print file");

    // Formatter
    println!("{}", 1);

    // Multiple formatters
    println!("{} is from {}", "Ivan", "Nigeria");

    // Positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Ivan", "Nigeria", "code"
    );

    // Named arguments
    println!(
        "{name} likes to {fun} and likes to {hobby}",
        name = "Ivan",
        fun = "code",
        hobby = "read"
    );

    // Placeholder traits
    println!("Binary: {:b} Hexadecimal: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder tuple-ish
    println!("{:?}", (12, true, "Hello"));
}
