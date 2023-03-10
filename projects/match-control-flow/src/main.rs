#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
        println!("State from {:?}", state);
        25
        }
    }
}


fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => {
                println!("{:?}", i);
                Some(i + 1)
            }
        }
            None => None
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);


    println!("{:?}", six);
    println!("{:?}", none);
}
