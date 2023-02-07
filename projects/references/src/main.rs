fn main() {
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    println!("The length of {} is {}", s1, len);

    // Mutable references
    let mut s2 = String::from("Hey");
    change(&mut s2);

    println!("Mutable reference gives {}", s2);

    let r1 = &mut s2;
    println!("{}", r1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}


