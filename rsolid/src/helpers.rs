use crate::*;

pub fn pentagon<R: Into<Length>>(radius: R) -> Circle {
    circle(radius).fragment_resolution(5)
}

pub fn hexagon<R: Into<Length>>(radius: R) -> Circle {
    circle(radius).fragment_resolution(6)
}

pub fn octagon<R: Into<Length>>(radius: R) -> Circle {
    circle(radius).fragment_resolution(8)
}
