use super::{catmull_rom, cubic, points::Points, quadratic, Command};

#[inline]
pub fn render(commands: &[Command], fragment_count: u64, points: &mut Points) {
    Render {
        fragment_count,
        points,
    }
    .apply(commands);
}

struct Render<'a> {
    fragment_count: u64,
    points: &'a mut Points,
}

impl Render<'_> {
    #[inline]
    fn apply(&mut self, commands: &[Command]) {
        for (idx, command) in commands.iter().enumerate() {
            match command {
                Command::Line { start, end } => {
                    self.points.push(*start);
                    self.points.push(*end);
                }
                Command::CubicBezierCurve {
                    start,
                    end,
                    start_control,
                    end_control,
                } => {
                    let n = self.fragment_count;
                    let steps = 0..=n;

                    self.points.extend(cubic::curve(
                        *start,
                        *start_control,
                        *end,
                        *end_control,
                        n,
                        steps,
                    ));
                }
                Command::SmoothBezierCurve {
                    start,
                    end,
                    end_control,
                } => todo!(),
                Command::QuadraticBezierCurve {
                    start,
                    end,
                    control,
                } => {
                    let n = self.fragment_count;
                    let steps = 0..=n;

                    self.points
                        .extend(quadratic::curve(*start, *control, *end, n, steps));
                }
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

                    self.points.extend(catmull_rom::curve(
                        *prev, *start, *end, *next, tension.0, n, steps,
                    ));
                }
                Command::Stroke { .. } => {
                    // TODO should this apply to polygons?
                    continue;
                }
            }
        }
    }
}
