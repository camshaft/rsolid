use rsolid::*;

fn main() {
    let c = cube([10, 20, 30]).down(5).rotate([45, 45, 45]);
    let s = sphere(10).fwd(5).dbg();

    let d = c - s;

    println!("{d}");
}
