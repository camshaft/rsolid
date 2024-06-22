use rsolid::*;

fn main() {
    let c = cube([10, 20, 30]);
    let s = sphere(10);

    let d = c - s;

    println!("{d}");
}
