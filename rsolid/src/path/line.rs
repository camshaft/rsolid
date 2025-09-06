use super::{catmull_rom, cubic, points::Points, quadratic, Command, Path};
use crate::{types as t, IntoObject, Object};

#[derive(Debug)]
pub struct Line {
    path: Path,
    width: t::Length,
}

impl Line {
    pub fn new(path: Path, width: t::Length) -> Self {
        Self { path, width }
    }
}

impl crate::scad::Scad for Line {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let width = (self.width.0 * 0.5).into();
        self.path
            .render_scad(f, |commands, fragment_count, points| {
                render(commands, width, fragment_count, points)
            })
    }
}

impl IntoObject<2> for Line {
    #[inline]
    fn into_object(self) -> crate::Object<2> {
        Object::new(self)
    }
}

impl ::core::fmt::Display for Line {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

#[inline]
pub fn render(commands: &[Command], width: t::Length, fragment_count: u64, points: &mut Points) {
    Render {
        fragment_count,
        width,
        points,
        prev_angle: 0.into(),
    }
    .apply(commands);
}

struct Render<'a> {
    fragment_count: u64,
    width: t::Length,
    points: &'a mut Points,
    prev_angle: t::Angle,
}

impl Render<'_> {
    #[inline]
    fn apply(&mut self, commands: &[Command]) {
        self.apply_fwd(commands);
        self.apply_back(commands);
    }

    #[inline]
    fn apply_fwd(&mut self, commands: &[Command]) {
        for (idx, command) in commands.iter().enumerate() {
            match command {
                Command::Line { start, end } => self.push_line(*start, *end),
                Command::CubicBezierCurve {
                    start,
                    end,
                    start_control,
                    end_control,
                } => todo!(),
                Command::SmoothBezierCurve {
                    start,
                    end,
                    end_control,
                } => todo!(),
                Command::QuadraticBezierCurve {
                    start,
                    end,
                    control,
                } => todo!(),
                Command::SmoothQuadraticBezierCurve { start, end } => todo!(),
                Command::EllipticalArc {
                    start,
                    end,
                    radii,
                    angle,
                    size,
                    sweep,
                } => todo!(),
                Command::CatmullRom {
                    start,
                    end,
                    tension,
                } => {
                    let n = self.fragment_count;
                    let steps = 0..=n;

                    let prev = commands[idx.checked_sub(1).unwrap_or(commands.len() - 1)].start();
                    let next = commands.get(idx + 1).unwrap_or_else(|| &commands[0]).end();

                    self.extend(
                        *start,
                        catmull_rom::curve(*prev, *start, *end, *next, tension.0, n, steps),
                    );
                }
                Command::Stroke { next, .. } => {
                    self.width = (next.0 * 0.5).into();
                }
            }
        }
    }

    #[inline]
    fn apply_back(&mut self, commands: &[Command]) {
        let mut cursor = t::Length2::from(0);
        for (idx, command) in commands.iter().enumerate().rev() {
            match command {
                Command::Line { start, end } => self.push_line(*end, *start),
                Command::CubicBezierCurve {
                    start,
                    end,
                    start_control,
                    end_control,
                } => todo!(),
                Command::SmoothBezierCurve {
                    start,
                    end,
                    end_control,
                } => todo!(),
                Command::QuadraticBezierCurve {
                    start,
                    end,
                    control,
                } => todo!(),
                Command::SmoothQuadraticBezierCurve { start, end } => todo!(),
                Command::EllipticalArc {
                    start,
                    end,
                    radii,
                    angle,
                    size,
                    sweep,
                } => todo!(),
                Command::CatmullRom {
                    start,
                    end,
                    tension,
                } => {
                    let n = self.fragment_count;
                    let steps = 0..=n;

                    let prev = commands[idx.checked_sub(1).unwrap_or(commands.len() - 1)].start();
                    let next = commands.get(idx + 1).unwrap_or_else(|| &commands[0]).end();

                    self.extend(
                        *start,
                        catmull_rom::curve(*next, *end, *start, *prev, tension.0, n, steps),
                    );
                }
                Command::Stroke { prev, .. } => {
                    self.width = (prev.0 * 0.5).into();
                }
            }
        }
    }

    #[inline]
    fn push_line(&mut self, start: t::Length2, end: t::Length2) {
        let slope = end - start;
        let new_angle = slope.angle();
        let normal = {
            let angle = new_angle.0 + core::f64::consts::FRAC_PI_2;
            let normal: t::Length2 = [angle.cos(), angle.sin()].into();
            normal
        };
        let normal = normal * self.width.0;

        let diff = (new_angle - self.prev_angle).0.abs();
        if diff > core::f64::consts::PI {
            eprintln!(
                "[{start:?} {end:?}] {} -> {} = {:?} {:?}",
                self.prev_angle,
                new_angle,
                new_angle - self.prev_angle,
                self.points.last(),
            );
        } else {
            self.points.push(start + normal);
        }

        self.points.push(end + normal);
        self.prev_angle = new_angle;
    }

    #[inline]
    fn extend(&mut self, mut start: t::Length2, points: impl Iterator<Item = t::Length2>) {
        for end in points {
            self.push_line(start, end);
            start = end;
        }
    }
}
