use rsolid::*;

fn main() {
    let c = cube([10, 20, 30]);
    let s = sphere(10) >> dbg();

    let d = c - s;

    let bg = cylinder(30, 4) >> bg();

    println!("{}", d + bg);
}
