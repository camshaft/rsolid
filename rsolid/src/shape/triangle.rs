use crate::*;

pub fn right(width: f64, height: f64) -> Object<2> {
    let angle = (height).atan2(width).to_degrees();
    let s = square([width, height]);
    let mask_v = width.max(height) * 2.0;
    let mask = square(mask_v) >> rotate_z(angle);
    let tri = s - mask;
    // center it and orient it
    tri >> back(height / 2.0) >> left(width / 2.0) >> mirror([1, 0, 0])
}

#[test]
fn right_test() {
    assert_2d_snapshot!(right(100.0, 200.0));
}

pub fn equilateral(length: f64) -> Object<2> {
    circle(3.0f64.sqrt() / 3.0 * length).fragment_resolution(3) >> rotate_z(-30)
}

#[test]
fn equilateral_test() {
    assert_2d_snapshot!(equilateral(80.0));
}
