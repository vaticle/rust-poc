extern crate rand;

use self::rand::random;

pub fn goodbye2() {
    let x: u8 = random();
    println!("{}", x);
}
