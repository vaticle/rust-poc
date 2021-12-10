extern crate lib1;
extern crate lib2;

fn main() {
    lib1::fruit::apple::hello1();
    lib1::fruit::banana::goodbye1();
    lib2::veg::carrot::hello2();
    lib2::veg::onion::goodbye2();
}

