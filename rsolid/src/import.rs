use super::*;
use core::fmt;
use scad::Scad;

#[derive(Debug)]
struct Import<const D: usize>(String);

impl<const D: usize> scad::Scad for Import<D> {
    #[inline]
    fn assign(&self, f: &mut scad::Formatter) -> scad::Assignment {
        f.emit(format!("import({:?})", self.0), scad::AssignmentType::Call)
    }
}

impl<const D: usize> fmt::Display for Import<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_scad())
    }
}

impl<const D: usize> IntoObject<D> for Import<D> {
    fn into_object(self) -> Object<D> {
        Object::new(self)
    }
}

macro_rules! import {
    ($name:ident, $dim:literal) => {
        pub fn $name<P: core::fmt::Display>(path: P) -> Object<$dim> {
            Import(path.to_string()).into_object()
        }

        #[macro_export]
        macro_rules! $name {
            ($path: literal) => {{
                // make sure it exists at compile time
                let _ = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/", $path));
                $crate::$name(concat!(env!("CARGO_MANIFEST_DIR"), "/src/", $path))
            }};
        }
    };
}

import!(svg, 2);
import!(stl, 3);
import!(amf, 3);
