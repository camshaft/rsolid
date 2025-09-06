use crate::scad::Scad;
use core::{fmt, ops};

impl Scad for bool {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        f.value(self)
    }
}

impl Scad for String {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        f.value(format_args!("{self:?}"))
    }
}

macro_rules! impl_simple {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
        pub struct $name(pub f64);

        impl From<f64> for $name {
            #[inline]
            fn from(value: f64) -> Self {
                Self(value)
            }
        }

        impl From<i64> for $name {
            #[inline]
            fn from(value: i64) -> Self {
                Self(value as f64)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Scad for $name {
            fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
                f.value(self)
            }
        }

        impl ops::Add for $name {
            type Output = $name;

            #[inline]
            fn add(self, rhs: $name) -> Self::Output {
                $name(self.0 + rhs.0)
            }
        }

        impl ops::AddAssign for $name {
            #[inline]
            fn add_assign(&mut self, rhs: $name) {
                self.0 += rhs.0;
            }
        }

        impl ops::Sub for $name {
            type Output = $name;

            #[inline]
            fn sub(self, rhs: $name) -> Self::Output {
                $name(self.0 - rhs.0)
            }
        }

        impl ops::SubAssign for $name {
            #[inline]
            fn sub_assign(&mut self, rhs: $name) {
                self.0 -= rhs.0;
            }
        }
    };
}

macro_rules! impl_measure_ops {
    ($name:ident) => {
        impl ops::Neg for $name {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                Self(-self.0)
            }
        }
    };
}

impl_simple!(Angle);
impl_measure_ops!(Angle);
impl_simple!(Length);
impl_measure_ops!(Length);
impl_simple!(Scalar);
impl_measure_ops!(Scalar);
impl_simple!(FragmentResolution);

macro_rules! impl_vec {
    ($name:ident, $vec:ident, $inner:ty, $count:literal) => {
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
        pub struct $name(pub [$inner; $count]);

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, concat!(stringify!($name), "["))?;
                for (idx, v) in self.0.iter().enumerate() {
                    if idx == 0 {
                        write!(f, "{v}")?;
                    } else {
                        write!(f, ", {v}")?;
                    }
                }
                write!(f, "]")
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[")?;
                for (idx, v) in self.0.iter().enumerate() {
                    if idx == 0 {
                        write!(f, "{v}")?;
                    } else {
                        write!(f, ",{v}")?;
                    }
                }
                write!(f, "]")
            }
        }

        impl Scad for $name {
            fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
                f.value(&self)
            }
        }

        impl From<f64> for $name {
            #[inline]
            fn from(value: f64) -> Self {
                Self([value.into(); $count])
            }
        }

        impl From<i64> for $name {
            #[inline]
            fn from(value: i64) -> Self {
                Self([value.into(); $count])
            }
        }

        impl From<$inner> for $name {
            #[inline]
            fn from(value: $inner) -> Self {
                Self([value; $count])
            }
        }

        impl From<[f64; $count]> for $name {
            #[inline]
            fn from(value: [f64; $count]) -> Self {
                Self(value.map(|v| v.into()))
            }
        }

        impl From<[i64; $count]> for $name {
            #[inline]
            fn from(value: [i64; $count]) -> Self {
                Self(value.map(|v| v.into()))
            }
        }

        impl From<[$inner; $count]> for $name {
            #[inline]
            fn from(value: [$inner; $count]) -> Self {
                Self(value)
            }
        }

        impl ops::Add for $name {
            type Output = $name;

            #[inline]
            fn add(mut self, rhs: $name) -> Self::Output {
                self += rhs;
                self
            }
        }

        impl ops::AddAssign for $name {
            #[inline]
            fn add_assign(&mut self, rhs: $name) {
                for (a, b) in self.0.iter_mut().zip(rhs.0) {
                    *a += b;
                }
            }
        }

        impl ops::Sub for $name {
            type Output = $name;

            #[inline]
            fn sub(mut self, rhs: $name) -> Self::Output {
                self -= rhs;
                self
            }
        }

        impl ops::SubAssign for $name {
            #[inline]
            fn sub_assign(&mut self, rhs: $name) {
                for (a, b) in self.0.iter_mut().zip(rhs.0) {
                    *a -= b;
                }
            }
        }

        impl ops::Mul<f64> for $name {
            type Output = $name;

            #[inline]
            fn mul(mut self, rhs: f64) -> Self::Output {
                for a in self.0.iter_mut() {
                    a.0 *= rhs;
                }
                self
            }
        }

        impl ops::Div<f64> for $name {
            type Output = $name;

            #[inline]
            fn div(mut self, rhs: f64) -> Self::Output {
                for a in self.0.iter_mut() {
                    a.0 /= rhs;
                }
                self
            }
        }

        impl Scad for std::vec::Vec<$name> {
            fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
                struct Fmt<'a>(&'a [$name]);

                impl<'a> fmt::Display for Fmt<'a> {
                    #[inline]
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "[")?;
                        for (idx, arg) in self.0.iter().enumerate() {
                            if idx != 0 {
                                write!(f, ",")?;
                            }
                            write!(f, "{arg}")?;
                        }
                        write!(f, "]")
                    }
                }

                f.value(Fmt(self))
            }
        }

        #[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
        pub struct $vec(pub std::vec::Vec<$name>);

        impl fmt::Display for $vec {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_list().entries(self.0.iter()).finish()
            }
        }

        impl Scad for $vec {
            fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
                self.0.assign(f)
            }
        }

        impl<T> std::iter::FromIterator<T> for $vec
        where
            $name: From<T>,
        {
            fn from_iter<I>(iter: I) -> Self
            where
                I: IntoIterator<Item = T>,
            {
                Self(iter.into_iter().map(|v| v.into()).collect())
            }
        }

        impl<I, T> From<I> for $vec
        where
            I: IntoIterator<Item = T>,
            $name: From<T>,
        {
            fn from(v: I) -> Self {
                Self::from_iter(v)
            }
        }
    };
}

impl_vec!(Angle2, VecAngle2, Angle, 2);
impl_vec!(Angle3, VecAngle3, Angle, 3);
impl_vec!(Length2, VecLength2, Length, 2);
impl_vec!(Length3, VecLength3, Length, 3);
impl_vec!(Scalar2, VecScalar2, Scalar, 2);
impl_vec!(Scalar3, VecScalar3, Scalar, 3);

impl Length {
    pub fn with_angle<A: Into<Angle>>(self, angle: A) -> Length2 {
        let angle = angle.into();
        let x = angle.0.to_radians().cos() * self.0;
        let y = angle.0.to_radians().sin() * self.0;
        [x, y].into()
    }
}

impl Length2 {
    #[inline]
    pub fn angle(self) -> Angle {
        self.0[1].0.atan2(self.0[0].0).into()
    }
}

impl From<Length2> for Length3 {
    fn from(value: Length2) -> Self {
        let [x, y] = value.0;
        [x, y, Length::default()].into()
    }
}
