mod m1 {
    pub mod host {
        pub fn add() {
            println!("Hello, world!");
        }
    }
}


mod m2 {
    use crate::m1::host;

    pub fn display() {
        host::add();
    }
}

use crate::m2 as other_m2;
use front_of_house::hosting;

fn main() {
    other_m2::display();
    hosting::add_to_waitlist();
}
