use crate::{
    scad::{self, Scad},
    ObjectExt, Operator,
};
use core::{fmt, ops};
use std::sync::Arc;

trait ObjectValue: 'static + Send + Sync + fmt::Debug + fmt::Display + Scad {}

impl<T> ObjectValue for T where T: 'static + Send + Sync + fmt::Debug + fmt::Display + Scad {}

#[derive(Clone)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Object<const DIMENSIONS: usize = 3>(Arc<dyn ObjectValue>);

impl<const DIMENSIONS: usize> Object<DIMENSIONS> {
    #[inline]
    pub fn new<T>(value: T) -> Self
    where
        T: 'static + Send + Sync + fmt::Debug + fmt::Display + Scad,
    {
        Self(Arc::new(value))
    }
}

impl<const DIMENSIONS: usize> fmt::Debug for Object<DIMENSIONS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<const DIMENSIONS: usize> fmt::Display for Object<DIMENSIONS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<const DIMENSIONS: usize> Scad for Object<DIMENSIONS> {
    fn assign(&self, f: &mut scad::Formatter) -> scad::Assignment {
        self.0.assign(f)
    }
}

impl<const DIMENSIONS: usize, T> ObjectExt<DIMENSIONS> for T where T: IntoObject<DIMENSIONS> + Sized {}

pub trait IntoObject<const DIMENSIONS: usize> {
    fn into_object(self) -> Object<DIMENSIONS>;
}

impl<const DIMENSIONS: usize> IntoObject<DIMENSIONS> for Object<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> Object<DIMENSIONS> {
        self
    }
}

impl<const DIMENSIONS: usize> IntoObject<DIMENSIONS> for &Object<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> Object<DIMENSIONS> {
        self.clone()
    }
}

impl<const DIMENSIONS: usize, F: Operator<DIMENSIONS>> ops::Shr<F> for Object<DIMENSIONS> {
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        f.apply(self)
    }
}

impl<const DIMENSIONS: usize, T: IntoObject<DIMENSIONS>> ops::Add<T> for Object<DIMENSIONS> {
    type Output = Self;

    fn add(self, other: T) -> Self {
        self.union(other)
    }
}

impl<const DIMENSIONS: usize, T: IntoObject<DIMENSIONS>> ops::AddAssign<T> for Object<DIMENSIONS> {
    fn add_assign(&mut self, rhs: T) {
        *self = self.clone().union(rhs);
    }
}

impl<const DIMENSIONS: usize, T: IntoObject<DIMENSIONS>> ops::BitOr<T> for Object<DIMENSIONS> {
    type Output = Self;

    fn bitor(self, other: T) -> Self {
        self.union(other)
    }
}

impl<const DIMENSIONS: usize, T: IntoObject<DIMENSIONS>> ops::BitOrAssign<T>
    for Object<DIMENSIONS>
{
    fn bitor_assign(&mut self, rhs: T) {
        *self = self.clone().union(rhs);
    }
}

impl<const DIMENSIONS: usize, T: IntoObject<DIMENSIONS>> ops::Sub<T> for Object<DIMENSIONS> {
    type Output = Self;

    fn sub(self, other: T) -> Self {
        self.difference(other)
    }
}

impl<const DIMENSIONS: usize, T: IntoObject<DIMENSIONS>> ops::SubAssign<T> for Object<DIMENSIONS> {
    fn sub_assign(&mut self, other: T) {
        *self = self.clone().difference(other)
    }
}

impl<const DIMENSIONS: usize, T: IntoObject<DIMENSIONS>> ops::BitAnd<T> for Object<DIMENSIONS> {
    type Output = Self;

    fn bitand(self, other: T) -> Self {
        self.intersection(other)
    }
}

impl<const DIMENSIONS: usize, T: IntoObject<DIMENSIONS>> ops::BitAndAssign<T>
    for Object<DIMENSIONS>
{
    fn bitand_assign(&mut self, other: T) {
        *self = self.clone().intersection(other);
    }
}
