mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn srve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    create::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {
    
}
