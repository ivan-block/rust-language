mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("You have been added to the waitlist!");
        }
        fn seat_at_table() {
            println!("Here is your seat, esteemed customer");
        }
    }

    mod serving {
        pub fn take_order() {
            println!("May i take your order, Sir");
        }

        pub fn serve_order() {
            println!("Here is your order, Madame");
        }

        pub fn take_payment() {
            println!("Here is your bill");
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
