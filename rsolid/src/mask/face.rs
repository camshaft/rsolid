use super::*;

pub fn cylinder(radius: impl Into<Length>, shape: impl IntoObject<2>) -> Object {
    let shape = shape.into_object() >> right(radius);
    shape >> rotate_extrude()
}

#[test]
fn cylinder_test() {
    assert_3d_snapshot!(cylinder(100.0, fillet(10.0)));
}

pub fn cube(width: f64, height: f64, shape: impl IntoObject<2>) -> Object {
    let shape = &shape.into_object();
    let a = edge(height, shape) >> xrot(90);
    let b = edge(width, shape) >> rotate([90, 0, 90]);
    let a1 = &a >> right(width / 2.0);
    let a2 = &a1 >> mirror([1, 0, 0]);
    let b1 = &b >> fwd(height / 2.0);
    let b2 = &b1 >> mirror([0, 1, 0]);

    a1 + a2 + b1 + b2
}

#[test]
fn cube_test() {
    assert_3d_snapshot!(cube(50.0, 100.0, fillet(10.0)));
}
