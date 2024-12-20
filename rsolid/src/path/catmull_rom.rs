use crate::types as t;

#[inline]
pub fn curve(
    p0: t::Length2,
    p1: t::Length2,
    p2: t::Length2,
    p3: t::Length2,
    tension: f64,
    n: u64,
    steps: impl Iterator<Item = u64>,
) -> impl Iterator<Item = t::Length2> {
    super::cubic::curve(
        p1,
        p1 + (p2 - p0) / 6.0 / tension,
        p2,
        p2 - (p3 - p1) / 6.0 / tension,
        n,
        steps,
    )
}

#[inline]
#[allow(dead_code)]
pub fn point(
    p0: t::Length2,
    p1: t::Length2,
    p2: t::Length2,
    p3: t::Length2,
    n: f64,
    t: f64,
    tension: f64,
) -> t::Length2 {
    super::cubic::point(
        p1,
        p1 + (p2 - p0) / 6.0 / tension,
        p2,
        p2 - (p3 - p1) / 6.0 / tension,
        n,
        t,
    )
}
