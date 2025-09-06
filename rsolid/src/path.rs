use core::fmt;

use crate::types as t;

mod catmull_rom;
mod cubic;
mod line;
mod points;
mod polygon;
mod quadratic;

#[derive(Clone, Debug, Default)]
pub struct Path {
    commands: Vec<Command>,
    cursor: t::Length2,
    angle: t::Angle,
    includes_curves: bool,
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

#[derive(Clone, Debug)]
enum Position {
    Absolute,
    Relative,
}

#[derive(Clone, Debug)]
enum ArcSize {
    Small,
    Large,
}

#[derive(Clone, Debug)]
enum ArcSweep {
    Clockwise,
    CounterClockwise,
}

#[derive(Clone, Debug)]
enum Command {
    Line {
        start: t::Length2,
        end: t::Length2,
    },
    CubicBezierCurve {
        start: t::Length2,
        end: t::Length2,
        start_control: t::Length2,
        end_control: t::Length2,
    },
    SmoothBezierCurve {
        start: t::Length2,
        end: t::Length2,
        end_control: t::Length2,
    },
    QuadraticBezierCurve {
        start: t::Length2,
        end: t::Length2,
        control: t::Length2,
    },
    SmoothQuadraticBezierCurve {
        start: t::Length2,
        end: t::Length2,
    },
    EllipticalArc {
        start: t::Length2,
        end: t::Length2,
        radii: t::Length2,
        angle: t::Angle,
        size: ArcSize,
        sweep: ArcSweep,
    },
    CatmullRom {
        start: t::Length2,
        end: t::Length2,
        tension: t::Scalar,
    },
    Stroke {
        prev: t::Length,
        next: t::Length,
    },
}

impl Command {
    #[inline]
    pub fn start(&self) -> &t::Length2 {
        match self {
            Command::Line { start, .. }
            | Command::CubicBezierCurve { start, .. }
            | Command::SmoothBezierCurve { start, .. }
            | Command::QuadraticBezierCurve { start, .. }
            | Command::SmoothQuadraticBezierCurve { start, .. }
            | Command::EllipticalArc { start, .. }
            | Command::CatmullRom { start, .. } => start,
            Command::Stroke { .. } => unreachable!(),
        }
    }

    #[inline]
    pub fn end(&self) -> &t::Length2 {
        match self {
            Command::Line { end, .. }
            | Command::CubicBezierCurve { end, .. }
            | Command::SmoothBezierCurve { end, .. }
            | Command::QuadraticBezierCurve { end, .. }
            | Command::SmoothQuadraticBezierCurve { end, .. }
            | Command::EllipticalArc { end, .. }
            | Command::CatmullRom { end, .. } => end,
            Command::Stroke { .. } => unreachable!(),
        }
    }
}

macro_rules! cmd {
    ($self:ident, $name:ident, $position:ident, $end:ident $(, $value:ident)*) => {{
        let start = $self.cursor;
        let $end = $self.resolve(Position::$position, $end.into());
        $self.push_cmd(Command::$name {
            start,
            $end,
            $(
                $value: $self.resolve(Position::$position, $value.into()),
            )*
        });
        $self
    }};
}

impl Path {
    #[inline]
    pub fn angle(&self) -> t::Angle {
        self.angle
    }

    #[inline]
    pub fn position(&self) -> t::Length2 {
        self.cursor
    }

    /// Move the current point to the coordinate `x`, `y`.
    ///
    /// Any subsequent coordinate pair(s) are interpreted as parameter(s) for implicit absolute
    /// [`line_to`] command(s).
    #[inline]
    pub fn move_to<P>(mut self, point: P) -> Self
    where
        P: Into<t::Length2>,
    {
        self.cursor = self.resolve(Position::Absolute, point.into());
        self
    }

    /// Move the current point by shifting the last known position of the path by `dx` along the
    /// x-axis and by `dy` along the y-axis.
    ///
    /// Any subsequent coordinate pair(s) are interpreted as parameter(s) for implicit absolute
    /// [`line_to`] command(s).
    #[inline]
    pub fn move_by<P>(mut self, dx_dy: P) -> Self
    where
        P: Into<t::Length2>,
    {
        self.cursor = self.resolve(Position::Relative, dx_dy.into());
        self
    }

    /// Moves forward by the given length at the current angle
    #[inline]
    pub fn move_fwd<P>(self, length: P) -> Self
    where
        P: Into<t::Length>,
    {
        let xy = length.into().with_angle(self.angle);
        self.move_by(xy)
    }

    /// Draw a line from the current point to the end point specified by `x`, `y`.
    ///
    /// Any subsequent coordinate pair(s) are interpreted as parameter(s) for implicit absolute
    /// [`line_to`] command(s).
    #[inline]
    pub fn line_to<P>(mut self, point: P) -> Self
    where
        P: Into<t::Length2>,
    {
        let end = point;
        cmd!(self, Line, Absolute, end)
    }

    /// Draw a line from the current point to the end point, which is the current point shifted
    /// by `dx` along the x-axis and `dy` along the y-axis.
    ///
    /// Any subsequent coordinate pair(s) are interpreted as parameter(s) for implicit absolute
    /// [`line_to`] command(s).
    #[inline]
    pub fn line_by<P>(mut self, dx_dy: P) -> Self
    where
        P: Into<t::Length2>,
    {
        let end = dx_dy;
        cmd!(self, Line, Relative, end)
    }

    /// Draws a line forward by the given length at the current angle
    #[inline]
    pub fn line_fwd<P>(self, length: P) -> Self
    where
        P: Into<t::Length>,
    {
        let xy = length.into().with_angle(self.angle);
        self.line_by(xy)
    }

    #[inline]
    pub fn quadratic_curve_to<P, C>(mut self, end: P, control: C) -> Self
    where
        P: Into<t::Length2>,
        C: Into<t::Length2>,
    {
        self.includes_curves = true;
        cmd!(self, QuadraticBezierCurve, Absolute, end, control)
    }

    #[inline]
    pub fn quadratic_curve_by<P, C>(mut self, end: P, control: C) -> Self
    where
        P: Into<t::Length2>,
        C: Into<t::Length2>,
    {
        self.includes_curves = true;
        cmd!(self, QuadraticBezierCurve, Relative, end, control)
    }

    #[inline]
    pub fn quadratic_curve_fwd<P, C>(mut self, length: P, control: C) -> Self
    where
        P: Into<t::Length>,
        C: Into<t::Length2>,
    {
        self.includes_curves = true;
        let end = length.into().with_angle(self.angle);
        cmd!(self, QuadraticBezierCurve, Relative, end, control)
    }

    #[inline]
    pub fn cubic_curve_to<P, C0, C1>(mut self, end: P, start_control: C0, end_control: C1) -> Self
    where
        P: Into<t::Length2>,
        C0: Into<t::Length2>,
        C1: Into<t::Length2>,
    {
        self.includes_curves = true;
        cmd!(
            self,
            CubicBezierCurve,
            Absolute,
            end,
            start_control,
            end_control
        )
    }

    #[inline]
    pub fn cubic_curve_by<P, C0, C1>(mut self, end: P, start_control: C0, end_control: C1) -> Self
    where
        P: Into<t::Length2>,
        C0: Into<t::Length2>,
        C1: Into<t::Length2>,
    {
        self.includes_curves = true;
        cmd!(
            self,
            CubicBezierCurve,
            Relative,
            end,
            start_control,
            end_control
        )
    }

    #[inline]
    pub fn cubic_curve_fwd<P, C0, C1>(
        mut self,
        length: P,
        start_control: C0,
        end_control: C1,
    ) -> Self
    where
        P: Into<t::Length>,
        C0: Into<t::Length2>,
        C1: Into<t::Length2>,
    {
        self.includes_curves = true;
        let end = length.into().with_angle(self.angle);
        cmd!(
            self,
            CubicBezierCurve,
            Relative,
            end,
            start_control,
            end_control
        )
    }

    #[inline]
    pub fn catmull_rom_to<P, T>(mut self, end: P, tension: T) -> Self
    where
        P: Into<t::Length2>,
        T: Into<t::Scalar>,
    {
        self.includes_curves = true;
        let start = self.cursor;
        let end = self.resolve(Position::Absolute, end.into());
        let tension = tension.into();
        self.push_cmd(Command::CatmullRom {
            start,
            end,
            tension,
        });
        self
    }

    #[inline]
    pub fn catmull_rom_by<P, T>(mut self, end: P, tension: T) -> Self
    where
        P: Into<t::Length2>,
        T: Into<t::Scalar>,
    {
        self.includes_curves = true;
        let start = self.cursor;
        let end = self.resolve(Position::Relative, end.into());
        let tension = tension.into();
        self.push_cmd(Command::CatmullRom {
            start,
            end,
            tension,
        });
        self
    }

    #[inline]
    pub fn catmull_rom_fwd<P, T>(mut self, length: P, tension: T) -> Self
    where
        P: Into<t::Length>,
        T: Into<t::Scalar>,
    {
        self.includes_curves = true;
        let start = self.cursor;
        let end = length.into().with_angle(self.angle);
        let end = self.resolve(Position::Relative, end);
        let tension = tension.into();
        self.push_cmd(Command::CatmullRom {
            start,
            end,
            tension,
        });
        self
    }

    /// Rotates by the provided angle
    #[inline]
    pub fn rotate<A>(mut self, angle: A) -> Self
    where
        A: Into<t::Angle>,
    {
        self.angle += angle.into();
        self
    }

    #[inline]
    pub fn into_line<W>(self, width: W) -> line::Line
    where
        W: Into<t::Length>,
    {
        line::Line::new(self, width.into())
    }

    #[inline]
    fn push_cmd(&mut self, command: Command) {
        let end = *command.end();
        self.commands.push(command);
        self.cursor = end;
    }

    #[inline]
    fn resolve(&self, position: Position, point: t::Length2) -> t::Length2 {
        match position {
            Position::Absolute => point,
            Position::Relative => self.cursor + point,
        }
    }

    #[inline]
    fn render_scad<R>(&self, f: &mut crate::scad::Formatter, render: R) -> crate::scad::Assignment
    where
        R: Fn(&[Command], u64, &mut points::Points),
    {
        use crate::scad::Scad;

        let fragment_count = 100;
        let preview_fragment_count = 20;
        let mut points = Default::default();
        render(&self.commands, fragment_count, &mut points);
        let full = points.assign(f);

        let arg = if self.includes_curves {
            points.clear();
            render(&self.commands, preview_fragment_count, &mut points);
            let preview = points.assign(f);
            f.value(format_args!("$preview?{preview}:{full}"))
        } else {
            full
        };

        f.call("polygon", [("points", Some(arg))], false)
    }
}

impl crate::scad::Scad for Path {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        self.render_scad(f, polygon::render)
    }
}

impl crate::IntoObject<2> for Path {
    #[inline]
    fn into_object(self) -> crate::Object<2> {
        crate::Object::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scad::Scad;

    #[test]
    fn linear_test() {
        let path = Path::default()
            .move_to([0, 0])
            .line_to([10, 10])
            .line_by([10, 0])
            .line_by([0, -10])
            .line_by([-10, 0])
            .line_by([0, 10])
            .line_to([0, 0]);

        let scad = path.to_scad();

        assert_2d_snapshot!(scad);
    }

    #[test]
    fn quadratic_test() {
        let mag = 0.5;

        let path = Path::default()
            .move_to([0., 5.])
            .quadratic_curve_by(5., [0.0, mag])
            .quadratic_curve_by([5., -5.], [mag, 0.0])
            .quadratic_curve_by(-5., [0., -mag])
            .quadratic_curve_by([-5., 5.], [-mag, 0.0]);

        let scad = path.to_scad();

        assert_2d_snapshot!(scad);
    }

    #[test]
    fn cubic_test() {
        let mag = 0.1;

        let path = Path::default()
            .move_to([0., 5.])
            .cubic_curve_by(5., [0.0, -mag], [0.0, mag])
            .cubic_curve_by([5., -5.], [-mag, 0.0], [mag, 0.0])
            .cubic_curve_by(-5., [0., mag], [0., -mag])
            .cubic_curve_by([-5., 5.], [mag, 0.0], [-mag, 0.0]);

        let scad = path.to_scad();

        assert_2d_snapshot!(scad);
    }

    #[test]
    fn catmull_rom_test() {
        let dx = 6.0;
        let dy = 5.0;

        let path = Path::default()
            .move_to([0., dy])
            .catmull_rom_by([dx, dy * 2.0], 0.7)
            .catmull_rom_by([dx, -dy * 2.0], 0.7)
            .catmull_rom_by([-dx, -dy], 0.7)
            .catmull_rom_by([-dx, dy], 0.7);

        let scad = path.to_scad();

        assert_2d_snapshot!(scad);
    }

    #[test]
    fn line_segment_test() {
        let mut path = Path::default().move_to(0);
        let dx = 10;
        let dy = 5;
        for _ in 0..2 {
            path = path.line_by([dx, dy]).line_by([dx, -dy]);
        }

        let scad = path.into_line(1.0).to_scad();

        assert_2d_snapshot!(scad);
    }

    #[test]
    fn catmull_rom_segment_test() {
        let mut path = Path::default().move_to(0);
        let dx = 10;
        let dy = 5;
        let tension = 1.0;
        for _ in 0..2 {
            path = path
                .catmull_rom_by([dx, dy], tension)
                .catmull_rom_by([dx, -dy], tension);
        }

        let scad = path.into_line(1.0).to_scad();

        assert_2d_snapshot!(scad);
    }
}
