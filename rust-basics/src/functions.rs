pub fn run() {
    greeting("Hello", "Ivan");
    println!("{}", add(5, 5));

    // Closure
    // Added benefit of closure is that we can freely add more numbers
    let n3 = 10;
    let c_sum = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C sum is: {}", c_sum(3, 3));
}

fn greeting(greet:&str, name:&str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}