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
        #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
        pub struct $name(pub [$inner; $count]);

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[")?;
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

        impl Scad for $name {
            fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
                let mut out = "[".to_string();
                for (idx, arg) in self.0.iter().enumerate() {
                    if idx != 0 {
                        out.push_str(", ");
                    }
                    out += &arg.assign(f).to_string();
                }
                out.push(']');
                f.value(out)
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

        #[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
        pub struct $vec(pub std::vec::Vec<$name>);

        impl fmt::Display for $vec {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_list().entries(self.0.iter()).finish()
            }
        }

        impl Scad for $vec {
            fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
                let mut out = "[".to_string();
                for (idx, arg) in self.0.iter().enumerate() {
                    if idx != 0 {
                        out.push_str(", ");
                    }
                    out += &arg.assign(f).to_string();
                }
                out.push(']');
                f.value(out)
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
