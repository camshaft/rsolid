use crate::types as t;

#[inline]
pub fn curve(
    p0: t::Length2,
    c0: t::Length2,
    p1: t::Length2,
    c1: t::Length2,
    n: u64,
    steps: impl Iterator<Item = u64>,
) -> impl Iterator<Item = t::Length2> {
    steps.map(move |t| point(p0, c0, p1, c1, n as f64, t as f64))
}

#[inline]
pub fn point(
    p0: t::Length2,
    c0: t::Length2,
    p1: t::Length2,
    c1: t::Length2,
    n: f64,
    t: f64,
) -> t::Length2 {
    let t0 = t / n;
    let t1 = (1. - t0).powi(3);
    let t2 = (1. - t0).powi(2);
    let t3 = t0.powi(2) * (1. - t0);
    let t4 = t0.powi(3);

    let dim = |p0: t::Length, c0: t::Length, c1: t::Length, p1: t::Length| {
        p0.0 * t1 + 3. * c0.0 * t0 * t2 + 3. * c1.0 * t3 + p1.0 * t4
    };

    let x = dim(p0.0[0], c0.0[0], c1.0[0], p1.0[0]);
    let y = dim(p0.0[1], c0.0[1], c1.0[1], p1.0[1]);

    [x, y].into()
}
