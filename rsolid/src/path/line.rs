use super::{points::Points, Command, Path};
use crate::types as t;

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
        let width = self.width;
        self.path
            .render_scad(f, |commands, fragment_count, points| {
                render(commands, width, fragment_count, points)
            })
    }
}

#[inline]
pub fn render(commands: &[Command], width: t::Length, fragment_count: u64, points: &mut Points) {
    Render {
        fragment_count,
        width,
        points,
    }
    .apply(commands);
}

struct Render<'a> {
    fragment_count: u64,
    width: t::Length,
    points: &'a mut Points,
}

impl Render<'_> {
    #[inline]
    fn apply(&mut self, commands: &[Command]) {
        self.apply_fwd(commands);
        // self.apply_back(commands);
    }

    #[inline]
    fn apply_fwd(&mut self, commands: &[Command]) {
        let mut cursor = t::Length2::from(0);
        for (idx, command) in commands.iter().enumerate() {
            match command {
                Command::Line { start, end } => {
                    let dx = end.0[0] - start.0[0];
                    let dy = end.0[1] - start.0[1];
                    dbg!([dx, dy]);
                    self.points.push(*start);
                    self.points.push(*end);
                    // cursor = *end;
                }
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
                } => todo!(),
            }
        }
    }
}
