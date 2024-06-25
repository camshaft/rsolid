/// Only renders the subtree in preview mode
#[derive(Clone, Copy, Default)]
#[must_use = "Objects must be returned in order to be rendered"]
pub struct InPreview<const DIMENSIONS: usize> {}

/// Only renders the subtree in preview mode
#[inline]
pub fn in_preview<const DIMENSIONS: usize>() -> InPreview<DIMENSIONS> {
    InPreview::default()
}

impl<const DIMENSIONS: usize> InPreview<DIMENSIONS> {}

impl<const DIMENSIONS: usize> ::core::fmt::Debug for InPreview<DIMENSIONS> {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut s = f.debug_struct("in_preview");
        s.finish()
    }
}

impl<const DIMENSIONS: usize> crate::scad::Scad for InPreview<DIMENSIONS> {
    fn assign(&self, f: &mut crate::scad::Formatter) -> crate::scad::Assignment {
        let name = f.module("() { if($preview) { children(); } }");
        let args = [];
        f.call(name, args, true)
    }
}

impl<const DIMENSIONS: usize> ::core::fmt::Display for InPreview<DIMENSIONS> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(&crate::scad::Scad::to_scad(self))
    }
}

impl<const DIMENSIONS: usize> crate::Operator<DIMENSIONS> for InPreview<DIMENSIONS> {
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
    for InPreview<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn add(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().add(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Sub<T>
    for InPreview<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn sub(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().sub(other.into_object())
    }
}

impl<T: crate::IntoObject<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::BitOr<T>
    for InPreview<DIMENSIONS>
{
    type Output = crate::Object<DIMENSIONS>;

    fn bitor(self, other: T) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object().bitor(other.into_object())
    }
}

impl<F: crate::Operator<DIMENSIONS>, const DIMENSIONS: usize> ::core::ops::Shr<F>
    for InPreview<DIMENSIONS>
{
    type Output = F::Output;

    fn shr(self, f: F) -> Self::Output {
        use crate::IntoObject as _;
        self.into_object() >> f
    }
}

impl<const DIMENSIONS: usize> From<InPreview<DIMENSIONS>> for crate::Object<DIMENSIONS> {
    #[inline]
    fn from(value: InPreview<DIMENSIONS>) -> Self {
        crate::Object::new(value)
    }
}

impl<const DIMENSIONS: usize> crate::IntoObject<DIMENSIONS> for InPreview<DIMENSIONS> {
    #[inline]
    fn into_object(self) -> crate::Object<DIMENSIONS> {
        crate::Object::new(self)
    }
}
