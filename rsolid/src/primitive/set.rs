/// Subtracts the 2nd child node from the first one (logical and not).
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Difference<const DIMENSIONS: usize> {}

/// Subtracts the 2nd child node from the first one (logical and not).
#[inline]
pub fn difference<const DIMENSIONS: usize>() -> Difference<DIMENSIONS> {
    Difference::default()
}

impl<const DIMENSIONS: usize> Difference<DIMENSIONS> {}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Difference<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("difference");
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Difference<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = "difference";
        let args = [];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for Difference<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Difference<DIMENSIONS> {
    type Output = crate::Object<DIMENSIONS>;

    fn apply(self, child: &crate::Object<DIMENSIONS>) -> Self::Output {
        let obj: crate::operator::Wrapped<DIMENSIONS, DIMENSIONS> = crate::operator::Wrapped {
            parent: self.into(),
            child: child.clone(),
        };
        crate::Object::new(obj)
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Add<T>
    for Difference<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Difference<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Difference<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Difference<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Difference<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Difference<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Difference<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Creates the intersection of all child nodes. This keeps the overlapping portion (logical and).
/// Only the area which is common or shared by all children is retained.
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Intersection<const DIMENSIONS: usize> {}

/// Creates the intersection of all child nodes. This keeps the overlapping portion (logical and).
/// Only the area which is common or shared by all children is retained.
#[inline]
pub fn intersection<const DIMENSIONS: usize>() -> Intersection<DIMENSIONS> {
    Intersection::default()
}

impl<const DIMENSIONS: usize> Intersection<DIMENSIONS> {}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Intersection<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("intersection");
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Intersection<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = "intersection";
        let args = [];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for Intersection<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Intersection<DIMENSIONS> {
    type Output = crate::Object<DIMENSIONS>;

    fn apply(self, child: &crate::Object<DIMENSIONS>) -> Self::Output {
        let obj: crate::operator::Wrapped<DIMENSIONS, DIMENSIONS> = crate::operator::Wrapped {
            parent: self.into(),
            child: child.clone(),
        };
        crate::Object::new(obj)
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Add<T>
    for Intersection<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Intersection<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Intersection<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Intersection<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Intersection<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Intersection<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Intersection<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}

/// Creates a union of all its child nodes. This is the sum of all children (logical or).
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct Union<const DIMENSIONS: usize> {}

/// Creates a union of all its child nodes. This is the sum of all children (logical or).
#[inline]
pub fn union<const DIMENSIONS: usize>() -> Union<DIMENSIONS> {
    Union::default()
}

impl<const DIMENSIONS: usize> Union<DIMENSIONS> {}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for Union<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("union");
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for Union<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = "union";
        let args = [];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for Union<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for Union<DIMENSIONS> {
    type Output = crate::Object<DIMENSIONS>;

    fn apply(self, child: &crate::Object<DIMENSIONS>) -> Self::Output {
        let obj: crate::operator::Wrapped<DIMENSIONS, DIMENSIONS> = crate::operator::Wrapped {
            parent: self.into(),
            child: child.clone(),
        };
        crate::Object::new(obj)
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Add<T>
    for Union<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for Union<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for Union<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for Union<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<Union<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: Union<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for Union<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}
