use crate::{
    scad::{self, Scad},
    IntoObject, Object,
};
use core::fmt;

pub trait Operator<const DIMENSIONS: usize> {
    type Output;

    fn apply(self, object: Object<DIMENSIONS>) -> Self::Output;
}

#[derive(Clone, Debug)]
pub struct Wrapped<const DIMENSIONS_IN: usize, const DIMENSIONS_OUT: usize> {
    pub parent: Object<DIMENSIONS_OUT>,
    pub child: Object<DIMENSIONS_IN>,
}

impl<const DIMENSIONS_IN: usize, const DIMENSIONS_OUT: usize> fmt::Display
    for Wrapped<DIMENSIONS_IN, DIMENSIONS_OUT>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_scad())
    }
}

impl<const DIMENSIONS_IN: usize, const DIMENSIONS_OUT: usize> Scad
    for Wrapped<DIMENSIONS_IN, DIMENSIONS_OUT>
{
    fn assign(&self, f: &mut scad::Formatter) -> scad::Assignment {
        let child = self.child.assign(f);
        let parent = self.parent.assign(f);
        f.emit(format_args!("{parent} {child}"), scad::AssignmentType::Call)
    }
}

impl<const DIMENSIONS_IN: usize, const DIMENSIONS_OUT: usize> IntoObject<DIMENSIONS_OUT>
    for Wrapped<DIMENSIONS_IN, DIMENSIONS_OUT>
{
    fn into_object(self) -> Object<DIMENSIONS_OUT> {
        Object::new(self)
    }
}
