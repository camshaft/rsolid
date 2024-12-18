use crate::*;

pub mod face;

fn preview_padding(width: f64, height: f64) -> Object<2> {
    let padding = 0.01;
    let u = square([width, padding]).center(true) >> fwd(height / 2.0);
    let o = square([padding, height]).center(true) >> right(width / 2.0);

    // only show the padding in preview mode - the regular render doesn't have artifacts
    (u + o) >> in_preview()
}

pub fn fillet(rounding: f64) -> Object<2> {
    let c = circle(rounding) >> back(rounding / 2.0) >> left(rounding / 2.0);
    let s = square(rounding).center(true);
    let padding = preview_padding(rounding, rounding);

    (s - c + padding) >> back(rounding / 2.0) >> left(rounding / 2.0)
}

#[test]
fn fillet_test() {
    assert_2d_snapshot!(fillet(100.0));
}

pub fn chamfer(width: f64, height: f64) -> Object<2> {
    let tri = triangle::right(width, height) >> rotate_z(180);
    let padding = preview_padding(width, height);

    (tri + padding) >> back(height / 2.0) >> left(width / 2.0)
}

#[test]
fn chamfer_test() {
    assert_2d_snapshot!(chamfer(50.0, 100.0));
}

pub fn edge(length: f64, shape: impl IntoObject<2>) -> Object {
    let shape = &shape.into_object();
    shape >> linear_extrude(length).center(true)
}

#[test]
fn edge_test() {
    assert_3d_snapshot!(edge(100.0, fillet(10.0)));
}
