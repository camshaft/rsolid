use rsolid::*;

fn main() {
    // TODO set global fn

    println!("{}", cube([10, 20, 30]).difference(sphere(10)));
}
