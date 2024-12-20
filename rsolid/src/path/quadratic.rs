use crate::types as t;

#[inline]
pub fn curve(
    p0: t::Length2,
    c: t::Length2,
    p1: t::Length2,
    n: u64,
    steps: impl Iterator<Item = u64>,
) -> impl Iterator<Item = t::Length2> {
    steps.map(move |t| point(p0, c, p1, n as f64, t as f64))
}

#[inline]
pub fn point(p0: t::Length2, c: t::Length2, p1: t::Length2, n: f64, t: f64) -> t::Length2 {
    let t0 = t / n;
    let t1 = (1. - t0).powi(2);
    let t2 = t0.powi(2);

    let dim = |p0: t::Length, c: t::Length, p1: t::Length| {
        p0.0 * t1 + 2. * c.0 * t0 * (1. - t0) + p1.0 * t2
    };

    let x = dim(p0.0[0], c.0[0], p1.0[0]);
    let y = dim(p0.0[1], c.0[1], p1.0[1]);

    [x, y].into()
}
