use rsolid::*;

fn wheel() -> Object {
    cylinder(15, 35).center(true).into()
}

fn axle() -> Object {
    let width = 120;
    let a = cylinder(width, 10).center(true) >> yrot(90);

    let w1 = wheel() >> yrot(90) >> left(width / 2);
    let w2 = wheel() >> yrot(90) >> right(width / 2);

    w1 + w2 + a
}

fn torso() -> Object {
    let bottom = cube([100, 250, 50]).center(true);
    let top = cube([80, 100, 60]).center(true);

    let window_cube = cube([200, 55, 50]).center(true).down(10);
    // TODO assign ops
    //    top -= window_cube + window_cube.rotate(0, 0, 90);

    // TODO avoid clone?
    let top = top - (window_cube.clone() + window_cube.rotate([0, 0, 90]));

    bottom + top.up(50)
}

fn car() -> Object {
    let t = torso();

    let front_axle = axle().down(20).back(80);
    let rear_axle = axle().down(20).fwd(80);

    t + front_axle + rear_axle
}

fn main() {
    println!("{}", car());
}
